use anyhow::Context;
use miasma::{MiasmaConfig, new_miasma_router};
use std::sync::LazyLock;

// TODO: add async method to check version and report to user if a newer version can be installed
// TODO: auto update cargo pacakge version in CD

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            let app = new_miasma_router(&CONFIG);

            let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
            let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .with_context(|| format!("could not bind to {addr}"))?;

            eprintln!(
                "Listening on '{addr}' with {} max in-flight requests...",
                CONFIG.max_in_flight
            );
            eprintln!(
                "Serving poisoned training data from '{}' with {} nested links per response...",
                CONFIG.poison_source, CONFIG.link_count
            );

            axum::serve(listener, app)
                .await
                .with_context(|| "server exited with an unexpected error")
        })
}
