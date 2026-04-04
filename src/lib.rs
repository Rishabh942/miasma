mod app;
mod config;
mod routes;
mod utils;
mod version_check;

use routes::new_miasma_router;

pub use app::Miasma;
pub use config::MiasmaConfig;
pub use version_check::check_for_new_version;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (github.com/austin-weeks/miasma)"
);
