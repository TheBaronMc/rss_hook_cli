use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a link between a RSS Flux and a webhook
    Create {
        flux_id: String,
        webhook_id: String
    },
    /// List all links
    Ls {
        flux_id: Option<u64>,
        webhook_id: Option<u64>
    },
    /// Remove a link
    Del {
        flux_id: u64,
        webhook_id: u64
    }
}