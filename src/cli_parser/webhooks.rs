use clap::Subcommand;
use super::BANNER;

#[derive(Subcommand)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
pub enum Commands {
    /// Insert a new Webhook
    Add {
        webhook_url: String
    },
    /// Remove a registered Webhook
    Del {
        webhook_id: u64
    },
    /// Update a registered Webhook
    Update {
        webhook_id: u64,
        webhook_url: String
    },
    /// List all webhooks
    List { }
}