use log::trace;
use reqwest::Client;
use warp::http::header::SET_COOKIE;
use warp::http::request::Parts;
use warp::hyper::{self, Body, Response};

use crate::error;

pub type Request = hyper::Request<Body>;

#[derive(Clone)]
pub(crate) struct HttpsClient {
    client: Client,
}

impl HttpsClient {
    pub(crate) fn new() -> Self {
        let client = Client::new();
        HttpsClient { client }
    }

    pub(crate) async fn request(&self, request: Request) -> Result<Response<Body>, error::Error> {
        let (parts, body) = request.into_parts();

        let Parts {
            method,
            uri,
            headers,
            ..
        } = parts;

        let url = reqwest::Url::parse(&uri.to_string())?;
        let body = reqwest::Body::wrap_stream(body);
        let request = self
            .client
            .request(method, url)
            .headers(headers)
            .body(body)
            .build()?;

        trace!("Sending request");
        let response = self.client.execute(request).await?;
        trace!("Got response");

        // Have to assign SET_COOKIE to a local variable here to avoid
        // https://github.com/rust-lang/rust-clippy/issues/3825
        let set_cookie = SET_COOKIE;
        let response_headers = response.headers().iter().filter(|(k, _)| k != &set_cookie);

        let mut builder = Response::builder();

        for (key, value) in response_headers {
            builder = builder.header(key, value);
        }

        let response = builder
            .status(response.status())
            .body(Body::wrap_stream(response.bytes_stream()))?;

        Ok(response)
    }
}
