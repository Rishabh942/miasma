mod app;
mod config;
mod routes;
mod utils;
mod version_check;

use routes::QueryParams;
use routes::new_miasma_router;

pub use app::Miasma;
pub use config::MiasmaConfig;
pub use version_check::check_for_new_version;

use bytes::Bytes;
use futures::Stream;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (github.com/austin-weeks/miasma)"
);

/// Alias for Stream of `Result<Bytes, E>`
pub trait MiasmaStream<E = anyhow::Error>: Stream<Item = Result<Bytes, E>> {}
impl<T, E> MiasmaStream<E> for T where T: Stream<Item = Result<Bytes, E>> {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, header::RETRY_AFTER},
    };
    use std::sync::LazyLock;
    use tower::ServiceExt;

    static TEST_CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(|| MiasmaConfig {
        port: 9999,
        host: "localhost".to_string(),
        #[cfg(unix)]
        unix_socket: None,
        max_in_flight: 1,
        link_prefix: "/".parse().unwrap(),
        link_count: 1,
        max_depth: crate::config::MaxDepth(None),
        force_gzip: false,
        unsafe_allow_html: false,
        poison_source: "http://example.com/".parse().unwrap(),
    });

    // This hits the poison source over the network; move to an integration test suite eventually.
    #[tokio::test]
    async fn fallback_route_works() {
        let app = new_miasma_router(&TEST_CONFIG);

        let response = app
            .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // could be 500 if the network is down or 200 if it works, but shouldn't be 404.
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn rate_limiting_returns_429() {
        let app = new_miasma_router(&TEST_CONFIG);
        let req1 = Request::builder().uri("/foo").body(Body::empty()).unwrap();
        let req2 = Request::builder().uri("/foo").body(Body::empty()).unwrap();

        let (res1, res2) = tokio::join!(app.clone().oneshot(req1), app.oneshot(req2));

        let res1 = res1.unwrap();
        let res2 = res2.unwrap();

        let r429 = if res1.status() == StatusCode::TOO_MANY_REQUESTS {
            res1
        } else if res2.status() == StatusCode::TOO_MANY_REQUESTS {
            res2
        } else {
            panic!(
                "expected one 429, got {} and {}",
                res1.status(),
                res2.status()
            );
        };

        assert_eq!(r429.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            r429.headers()
                .get(RETRY_AFTER)
                .and_then(|v| v.to_str().ok()),
            Some("5")
        );
    }
}
