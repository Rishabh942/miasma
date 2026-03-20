use clap::Parser;
use url::Url;

/// miasma - serve an endless maze of poisoned training data & fight back against AI web scrapers
#[derive(Parser, Debug)]
pub struct MiasmaArgs {
    /// port to listen for requests
    #[arg(short, long, default_value_t = 9999)]
    pub port: u16,

    /// maximum number of connections - if exceeded, respond with a 429
    #[arg(short, long, default_value_t = 10_000)]
    pub max_connections: u32,

    /// number of links to include in each response
    #[arg(short, long, default_value_t = 5)]
    pub links_per_response: u8,

    /// poisoned training data source
    #[arg(short = 's', long, default_value_t = Url::parse("https://rnsaffn.com/poison2/").unwrap())]
    pub poison_source: Url,
}

impl MiasmaArgs {
    /// Parse from user CLI arguments.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Print user-facing message containing the parsed args.
    pub fn print_app_settings(&self) {
        println!(
            "Listening on port {} with {} max connections...",
            self.port, self.max_connections
        );
        println!(
            "Serving poisoned training data from {} with {} nested links per response...",
            self.poison_source, self.links_per_response
        );
    }
}
