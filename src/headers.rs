use warp::http::header::{HeaderName, CONNECTION};
use warp::http::HeaderMap;

pub(crate) fn remove_hop_by_hop_headers(headers: &mut HeaderMap) {
    let connection_headers = headers
        .get_all(CONNECTION)
        .iter()
        .flat_map(|value| value.as_bytes().split(|byte| *byte == b','))
        .filter_map(|name| HeaderName::from_bytes(name.trim_ascii()).ok())
        .collect::<Vec<_>>();

    for name in connection_headers {
        headers.remove(name);
    }

    for name in [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "proxy-connection",
        "te",
        "trailer",
        "transfer-encoding",
        "upgrade",
    ] {
        headers.remove(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::HeaderValue;

    #[test]
    fn test_standard_hop_by_hop_headers() {
        let mut headers = HeaderMap::new();
        for name in [
            "connection",
            "keep-alive",
            "proxy-authenticate",
            "proxy-authorization",
            "proxy-connection",
            "te",
            "trailer",
            "transfer-encoding",
            "upgrade",
        ] {
            headers.append(name, HeaderValue::from_static("value"));
            headers.append(name, HeaderValue::from_static("another value"));
        }
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("content-length", HeaderValue::from_static("42"));
        headers.insert("authorization", HeaderValue::from_static("Bearer example"));
        headers.append("x-example", HeaderValue::from_static("first"));
        headers.append("x-example", HeaderValue::from_static("second"));

        remove_hop_by_hop_headers(&mut headers);

        assert_eq!(headers.len(), 5);
        assert_eq!(headers["content-type"], "application/json");
        assert_eq!(headers["content-length"], "42");
        assert_eq!(headers["authorization"], "Bearer example");
        assert_eq!(headers.get_all("x-example").iter().count(), 2);
    }

    #[test]
    fn test_all_connection_values_and_case_insensitive_names() {
        let mut headers = HeaderMap::new();
        headers.append(
            CONNECTION,
            HeaderValue::from_static(" X-First, keep-alive "),
        );
        headers.append(
            CONNECTION,
            HeaderValue::from_static("Connection,\tX-Second, CONTENT-LENGTH"),
        );
        headers.insert("x-first", HeaderValue::from_static("private"));
        headers.append("x-second", HeaderValue::from_static("private"));
        headers.append("x-second", HeaderValue::from_static("also private"));
        headers.insert("content-length", HeaderValue::from_static("42"));
        headers.insert("x-preserved", HeaderValue::from_static("public"));

        remove_hop_by_hop_headers(&mut headers);

        assert_eq!(headers.len(), 1);
        assert_eq!(headers["x-preserved"], "public");
    }

    #[test]
    fn test_invalid_connection_tokens_do_not_hide_valid_names() {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONNECTION,
            HeaderValue::from_bytes(b", invalid token, x-first, \xff, x-second,").unwrap(),
        );
        headers.insert("x-first", HeaderValue::from_static("private"));
        headers.insert("x-second", HeaderValue::from_static("private"));

        remove_hop_by_hop_headers(&mut headers);

        assert!(headers.is_empty());
    }
}
