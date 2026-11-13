use futures_util::stream::TryStreamExt;
use log::{error, trace};
use warp::http::header::{HeaderName, COOKIE, HOST, VIA};
use warp::http::{HeaderMap, HeaderValue, Method};
use warp::{header, Buf, Filter, Rejection, Stream};

use crate::client;
use crate::error;
use crate::filters;

pub(crate) fn proxied_request(
    host: String,
) -> impl Filter<Extract = (client::Request,), Error = Rejection> + Clone {
    warp::method()
        .and(filters::url_path())
        .and(header::headers_cloned())
        .and(warp::body::stream())
        .and_then(move |method, url, headers, body| {
            create_request(method, url, headers, body, host.clone())
        })
}

async fn create_request(
    method: Method,
    url: url::Url,
    mut headers: HeaderMap,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Send + Sync + 'static,
    host: String,
) -> Result<client::Request, Rejection> {
    crate::headers::remove_hop_by_hop_headers(&mut headers);

    headers.remove(COOKIE);
    headers.remove(HOST);

    let via_value = format!("2.0 {}", host);
    append_or_insert_header(&mut headers, VIA, &via_value)?;

    let body =
        reqwest::Body::wrap_stream(body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())));
    let mut request = reqwest::Request::new(method, url);
    *request.headers_mut() = headers;
    *request.body_mut() = Some(body);

    trace!("Created request");

    Ok(request)
}

fn append_or_insert_header(
    headers: &mut HeaderMap,
    header_name: HeaderName,
    value: &str,
) -> Result<(), error::Error> {
    if let Some(header_value) = headers.get_mut(&header_name) {
        let via_value = header_value.to_str().unwrap_or_else(|e| {
            error!(
                "Error converting `{}` header value to str: {}",
                header_name, e
            );
            Default::default()
        });

        if via_value.is_empty() {
            *header_value = HeaderValue::from_str(value)?;
        } else {
            *header_value = HeaderValue::from_str(&format!("{}, {}", via_value, value))?;
        };
    } else {
        headers.insert(&header_name, warp::http::HeaderValue::from_str(value)?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_request_filters_connection_headers() {
        let mut headers = HeaderMap::new();
        headers.append("connection", HeaderValue::from_static("x-first"));
        headers.append("connection", HeaderValue::from_static("X-Second"));
        headers.insert("x-first", HeaderValue::from_static("private"));
        headers.insert("x-second", HeaderValue::from_static("private"));
        headers.insert("transfer-encoding", HeaderValue::from_static("chunked"));
        headers.insert("cookie", HeaderValue::from_static("session=example"));
        headers.insert("host", HeaderValue::from_static("proxy.example.org"));
        headers.insert("authorization", HeaderValue::from_static("Bearer example"));
        let body = futures_util::stream::empty::<Result<&'static [u8], warp::Error>>();
        let request = create_request(
            Method::GET,
            url::Url::parse("https://example.org/").unwrap(),
            headers,
            body,
            "proxy.example.org".to_owned(),
        )
        .await
        .unwrap();

        for name in [
            "connection",
            "x-first",
            "x-second",
            "transfer-encoding",
            "cookie",
            "host",
        ] {
            assert!(!request.headers().contains_key(name), "forwarded {name}");
        }
        assert_eq!(request.headers()["authorization"], "Bearer example");
        assert_eq!(request.headers()["via"], "2.0 proxy.example.org");
    }
}
