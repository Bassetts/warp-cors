use warp::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS,
    VARY,
};
use warp::http::HeaderValue;
use warp::Reply;

pub(crate) fn allow_origin(reply: impl warp::Reply, origin: Option<String>) -> impl warp::Reply {
    let mut reply = reply.into_response();
    let headers = reply.headers_mut();

    headers.remove(ACCESS_CONTROL_ALLOW_ORIGIN);
    headers.remove(ACCESS_CONTROL_ALLOW_CREDENTIALS);

    let varies_on_origin = headers
        .get_all(VARY)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(','))
        .any(|value| value.trim().eq_ignore_ascii_case("origin") || value.trim() == "*");
    if !varies_on_origin {
        headers.append(VARY, HeaderValue::from_static("Origin"));
    }

    match origin {
        None => {
            headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        }
        Some(origin) if is_valid_origin(&origin) => {
            if let Ok(origin) = HeaderValue::from_str(&origin) {
                headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin);
                headers.insert(
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    HeaderValue::from_static("true"),
                );
            }
        }
        Some(_) => {}
    }

    reply
}

fn is_valid_origin(origin: &str) -> bool {
    origin == "null"
        || url::Url::parse(origin).is_ok_and(|url| url.origin().ascii_serialization() == origin)
}

pub(crate) fn expose_all_headers(reply: impl warp::Reply) -> impl warp::Reply {
    let reply = reply.into_response();
    let response_headers = reply
        .headers()
        .keys()
        .map(|k| k.as_str())
        .collect::<Vec<&str>>()
        .join(", ");

    if response_headers.is_empty() {
        reply
    } else {
        warp::reply::with_header(reply, ACCESS_CONTROL_EXPOSE_HEADERS, response_headers)
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::hyper::Response;
    use warp::Reply;

    #[tokio::test]
    async fn test_allow_origin_without_origin() {
        let response = Response::builder()
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "https://upstream.example")
            .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body("")
            .unwrap();
        let response = allow_origin(response, None).into_response();
        assert_eq!(response.headers()[ACCESS_CONTROL_ALLOW_ORIGIN], "*");
        assert!(!response
            .headers()
            .contains_key(ACCESS_CONTROL_ALLOW_CREDENTIALS));
        assert_eq!(response.headers()[VARY], "Origin");
    }

    #[tokio::test]
    async fn test_allow_origin() {
        for origin in [
            "https://example.org",
            "http://localhost:3000",
            "https://[2001:db8::1]:8443",
            "null",
        ] {
            let response = Response::builder()
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "false")
                .body("")
                .unwrap();
            let response = allow_origin(response, Some(origin.to_owned())).into_response();
            assert_eq!(response.headers()[ACCESS_CONTROL_ALLOW_ORIGIN], origin);
            assert_eq!(response.headers()[ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
            assert_eq!(response.headers()[VARY], "Origin");
        }
    }

    #[tokio::test]
    async fn test_invalid_origins_do_not_grant_cors() {
        for origin in [
            "",
            "*",
            "localhost",
            "https:example.org",
            "https://example.org/",
            "https://example.org/path",
            "https://example.org?query=value",
            "https://example.org#fragment",
            "https://user:password@example.org",
            "https://example.org https://other.example",
            "https://example.org, https://other.example",
            "file:///tmp/app.html",
        ] {
            let response = Response::builder()
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
                .body("")
                .unwrap();
            let response = allow_origin(response, Some(origin.to_owned())).into_response();
            assert!(
                !response.headers().contains_key(ACCESS_CONTROL_ALLOW_ORIGIN),
                "unexpected CORS grant for {origin:?}"
            );
            assert!(!response
                .headers()
                .contains_key(ACCESS_CONTROL_ALLOW_CREDENTIALS));
            assert_eq!(response.headers()[VARY], "Origin");
        }
    }

    #[tokio::test]
    async fn test_vary_preserves_upstream_fields() {
        let response = Response::builder()
            .header(VARY, "Accept-Encoding, Accept-Language")
            .header(VARY, "Accept")
            .body("")
            .unwrap();
        let response =
            allow_origin(response, Some("https://example.org".to_owned())).into_response();
        let vary = response
            .headers()
            .get_all(VARY)
            .iter()
            .map(|value| value.to_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            vary,
            ["Accept-Encoding, Accept-Language", "Accept", "Origin"]
        );
    }

    #[tokio::test]
    async fn test_vary_does_not_duplicate_origin_or_wildcard() {
        for vary in ["Origin", "origin", "Accept-Encoding, ORIGIN", "*"] {
            let response = Response::builder().header(VARY, vary).body("").unwrap();
            let response = allow_origin(response, None).into_response();
            assert_eq!(response.headers().get_all(VARY).iter().count(), 1);
            assert_eq!(response.headers()[VARY], vary);
        }
    }

    #[tokio::test]
    async fn test_expose_all_headers() {
        let response = Response::builder().body("").unwrap();
        let response = expose_all_headers(response).into_response();
        assert!(response.headers().is_empty());

        let response = Response::builder()
            .header("origin", "localhost")
            .body("")
            .unwrap();
        let response = expose_all_headers(response).into_response();
        assert_eq!(
            response
                .headers()
                .get(ACCESS_CONTROL_EXPOSE_HEADERS)
                .unwrap(),
            "origin"
        );

        let response = Response::builder()
            .header("origin", "localhost")
            .header("x-test-header", "test")
            .body("")
            .unwrap();
        let response = expose_all_headers(response).into_response();
        assert_eq!(
            response
                .headers()
                .get(ACCESS_CONTROL_EXPOSE_HEADERS)
                .unwrap(),
            "origin, x-test-header"
        );
    }
}
