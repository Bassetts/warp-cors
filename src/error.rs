use std::fmt;

use log::error;
use serde::Serialize;
use warp::http;
use warp::hyper;
use warp::{Rejection, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub(crate) async fn recover(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(ref err) = err.find::<Error>() {
        let error = ErrorMessage {
            code: 500,
            message: err.to_string(),
        };

        error!("Recovering from error `{}`", error.message);

        return Ok(warp::reply::with_status(
            warp::reply::json(&error),
            warp::http::StatusCode::from_u16(error.code).unwrap(),
        ));
    }

    Err(err)
}

#[derive(Debug)]
pub(crate) enum Error {
    Http(http::Error),
    Hyper(hyper::Error),
    InvalidHeaderValue(hyper::header::InvalidHeaderValue),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Http(err) => err.fmt(f),
            Error::Hyper(err) => err.fmt(f),
            Error::InvalidHeaderValue(err) => err.fmt(f),
        }
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error::Http(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<hyper::header::InvalidHeaderValue> for Error {
    fn from(err: hyper::header::InvalidHeaderValue) -> Error {
        Error::InvalidHeaderValue(err)
    }
}

impl warp::reject::Reject for Error {}

impl From<Error> for warp::reject::Rejection {
    fn from(error: Error) -> warp::reject::Rejection {
        warp::reject::custom(error)
    }
}
