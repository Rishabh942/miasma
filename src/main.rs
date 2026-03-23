use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use miasma::routes;
use tokio::sync::Semaphore;

// TODO: add async method to check version and report to user if a newer version can be installed

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let in_flight_sem = Arc::new(Semaphore::new(miasma::CONFIG.max_in_flight as usize));

    let app = axum::Router::new().fallback(get(move || async move {
        routes::serve_poison(&miasma::CONFIG, in_flight_sem).await
    }));

    let addr = format!("{}:{}", miasma::CONFIG.host, miasma::CONFIG.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("could not bind to {addr}"))?;

    eprintln!(
        "Listening on '{addr}' with {} max in-flight requests...",
        miasma::CONFIG.max_in_flight
    );
    eprintln!(
        "Serving poisoned training data from '{}' with {} nested links per response...",
        miasma::CONFIG.poison_source,
        miasma::CONFIG.link_count
    );

    axum::serve(listener, app)
        .await
        .with_context(|| "server exited with an unexpected error")
}
