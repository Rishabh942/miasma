mod html_builder;
mod poison;

use std::sync::Arc;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use reqwest::header;
use tokio::sync::{Semaphore, TryAcquireError};

use crate::config::MiasmaConfig;

/// Miasma's poison serving trap.
pub async fn serve_poison(config: &'static MiasmaConfig, sem: Arc<Semaphore>) -> impl IntoResponse {
    let permit = match sem.try_acquire_owned() {
        Ok(p) => p,
        Err(e) => match e {
            TryAcquireError::NoPermits => {
                return Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .header(header::RETRY_AFTER, 5)
                    .body(Body::empty())
                    .expect("this should never fail");
            }
            TryAcquireError::Closed => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        },
    };

    let poison = match poison::fetch_poison(&config.poison_source).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error fetching from {}: {e}", config.poison_source);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let stream = html_builder::POSION_PAGE.build_html_stream(
        poison,
        config.link_count,
        &config.link_prefix,
        permit,
    );

    // TODO: gzip compress response to save on bandwith costs
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(Body::from_stream(stream))
        .unwrap_or_else(|e| {
            eprintln!("Failed to build poison route response: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}
