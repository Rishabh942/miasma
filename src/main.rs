use anyhow::Context;
use axum::routing::get;
use miasma::MiasmaArgs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = MiasmaArgs::parse();

    let app = axum::Router::new().route("/", get(|| async { "Hello from miasma!" }));

    // TODO: 'localhost' may not be the right addr to bind to... do some research
    let listener = tokio::net::TcpListener::bind(format!("localhost:{}", args.port))
        .await
        .with_context(|| format!("could not bind to port {}", args.port))?;

    args.print_app_settings();

    axum::serve(listener, app)
        .await
        .with_context(|| "server exited with an unexpected error")
}
