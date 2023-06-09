use clap::Subcommand;
use super::BANNER;

#[derive(Subcommand)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
pub enum Commands {
    /// Create a link between a RSS Flux and a webhook
    Create {
        flux_id: u64,
        webhook_id: u64
    },
    /// Remove a link
    Del {
        flux_id: u64,
        webhook_id: u64
    }
}