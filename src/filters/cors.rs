use warp::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS};
use warp::Reply;

pub(crate) fn allow_any_origin(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(reply, ACCESS_CONTROL_ALLOW_ORIGIN, "*")
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
    async fn test_allow_any_origin() {
        let response = Response::builder().body("").unwrap();
        let response = allow_any_origin(response).into_response();
        assert_eq!(
            response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
            "*"
        );
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
