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

        let status = response.status();
        let headers = response
            .headers()
            .iter()
            .filter(|(key, _)| *key != SET_COOKIE)
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>();
        let mut response = warp::reply::stream(response.bytes_stream()).into_response();
        *response.status_mut() = status;
        for (key, value) in headers {
            response.headers_mut().append(key, value);
        }

        Ok(response)
    }
}
