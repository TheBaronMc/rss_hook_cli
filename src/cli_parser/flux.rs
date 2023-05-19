use clap::Subcommand;
use super::BANNER;

#[derive(Subcommand)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
pub enum Commands {
    /// Insert a new RSS Flux
    Add {
        flux_url: String
    },
    /// Remove a registered RSS Flux
    Del {
        flux_id: u64
    },
    /// Update a registered RSS Flux
    Update {
        flux_id: u64,
        flux_url: String
    },
    /// List all RSS Flux
    Ls {},
    // List all webhooks hooked to a flux 
    Hooks {
        flux_id: u64,
    }
}