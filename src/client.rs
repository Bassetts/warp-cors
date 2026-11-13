use log::trace;
use reqwest::Client;
use warp::http::header::SET_COOKIE;
use warp::Reply;

use crate::error;

pub type Request = reqwest::Request;

#[derive(Clone)]
pub(crate) struct HttpsClient {
    client: Client,
}

impl HttpsClient {
    pub(crate) fn new() -> Self {
        let client = Client::new();
        HttpsClient { client }
    }

    pub(crate) async fn request(
        &self,
        request: Request,
    ) -> Result<warp::reply::Response, error::Error> {
        trace!("Sending request");
        let response = self.client.execute(request).await?;
        trace!("Got response");

        Ok(create_response(response))
    }
}

fn create_response(response: reqwest::Response) -> warp::reply::Response {
    let status = response.status();
    let mut headers = response.headers().clone();
    crate::headers::remove_hop_by_hop_headers(&mut headers);
    headers.remove(SET_COOKIE);

    let mut response = warp::reply::stream(response.bytes_stream()).into_response();
    *response.status_mut() = status;
    *response.headers_mut() = headers;
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::Filter;

    #[tokio::test]
    async fn test_response_filters_headers_and_preserves_body() {
        let route = warp::any().map(|| {
            let body = reqwest::Body::wrap_stream(futures_util::stream::iter([
                Ok::<_, std::io::Error>("proxied "),
                Ok("body"),
            ]));
            let response = warp::http::Response::builder()
                .status(201)
                .header("connection", "x-first")
                .header("connection", "X-Second")
                .header("x-first", "private")
                .header("x-second", "private")
                .header("transfer-encoding", "chunked")
                .header("set-cookie", "session=example")
                .header("content-type", "text/plain")
                .header("x-example", "first")
                .header("x-example", "second")
                .body(body)
                .unwrap();
            create_response(response.into())
        });
        let response = warp::test::request().reply(&route).await;

        assert_eq!(response.status(), 201);
        assert_eq!(response.body().as_ref(), b"proxied body");
        for name in [
            "connection",
            "x-first",
            "x-second",
            "transfer-encoding",
            "set-cookie",
        ] {
            assert!(!response.headers().contains_key(name), "forwarded {name}");
        }
        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(response.headers().get_all("x-example").iter().count(), 2);
    }
}
