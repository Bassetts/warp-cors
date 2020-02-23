use warp::header;
use warp::Filter;

pub(crate) fn has_header(
    header: &'static str,
) -> impl Filter<Extract = (), Error = warp::Rejection> + Copy {
    header::header::<String>(header).map(|_| ()).untuple_one()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_has_header() {
        let origin = has_header("origin");

        let request = warp::test::request().header("origin", "test");
        assert!(request.matches(&origin).await);

        let request = warp::test::request().header("host", "localhost");
        assert!(!request.matches(&origin).await);

        let request = warp::test::request();
        assert!(!request.matches(&origin).await);
    }
}
