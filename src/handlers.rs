use std::convert::Infallible;

use log::trace;
use warp::http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, VARY};
use warp::Reply;

use crate::client;

pub(crate) async fn preflight_request(
    method: String,
    headers: Option<String>,
) -> Result<impl warp::Reply, Infallible> {
    trace!("Preflight request received");
    let reply = warp::reply::with_header(warp::reply(), ACCESS_CONTROL_ALLOW_METHODS, method);
    let reply = match headers {
        Some(headers) => {
            warp::reply::with_header(reply, ACCESS_CONTROL_ALLOW_HEADERS, headers).into_response()
        }
        None => reply.into_response(),
    };
    let reply = warp::reply::with_header(
        reply,
        VARY,
        "Access-Control-Request-Method, Access-Control-Request-Headers",
    );
    Ok(reply)
}

pub(crate) async fn proxy_request(
    request: client::Request,
    client: client::HttpsClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    trace!("Proxying request");
    let response = client.request(request).await?;
    trace!("Returning response");
    Ok(response)
}
