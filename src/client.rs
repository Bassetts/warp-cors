use hyper_rustls::HttpsConnector;
use log::trace;
use warp::http::header::SET_COOKIE;
use warp::hyper::client::{Client, HttpConnector};
use warp::hyper::{self, Body, Response};

use crate::error;

pub type Request = hyper::Request<Body>;

#[derive(Clone)]
pub(crate) struct HttpsClient {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl HttpsClient {
    pub(crate) fn new() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        HttpsClient { client }
    }

    pub(crate) async fn request(&self, request: Request) -> Result<Response<Body>, error::Error> {
        trace!("Sending request");
        let mut response = self.client.request(request).await?;
        trace!("Got response");

        response.headers_mut().remove(SET_COOKIE);

        Ok(response)
    }
}
