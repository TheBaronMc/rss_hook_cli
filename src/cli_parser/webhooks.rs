use clap::Subcommand;

#[derive(Subcommand)]
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
    }
}