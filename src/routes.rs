use warp::{Filter, Rejection, Reply};

use crate::client::HttpsClient;
use crate::error;
use crate::filters;
use crate::handlers;

pub fn routes(host: String) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let client = HttpsClient::new();
    preflight()
        .or(proxy(host, client))
        .map(filters::allow_any_origin)
        .with(warp::log("warp_cors"))
        .recover(error::recover)
}

fn preflight() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    filters::is_url_path()
        .and(warp::options())
        .and(filters::has_header("access-control-request-method"))
        .and(filters::has_header("access-control-request-headers"))
        .and(filters::has_header("origin"))
        .and_then(handlers::preflight_request)
}

fn proxy(
    host: String,
    client: HttpsClient,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    filters::proxied_request(host)
        .and(filters::with_client(client))
        .and_then(handlers::proxy_request)
        .map(filters::expose_all_headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_preflight() {
        let preflight = preflight();

        let request = warp::test::request();
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request().method("OPTIONS");
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request()
            .method("OPTIONS")
            .header("access-control-request-method", "GET");
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request()
            .method("OPTIONS")
            .header("access-control-request-method", "GET")
            .header("access-control-request-headers", "Origin");
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request()
            .method("OPTIONS")
            .header("access-control-request-method", "GET")
            .header("origin", "localhost");
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request()
            .method("GET")
            .header("access-control-request-method", "GET")
            .header("access-control-request-headers", "Origin")
            .header("origin", "localhost");
        assert!(!request.matches(&preflight).await);

        let request = warp::test::request()
            .method("OPTIONS")
            .header("access-control-request-method", "GET")
            .header("access-control-request-headers", "Origin")
            .header("origin", "localhost");
        assert!(request.matches(&preflight).await);
    }
}
