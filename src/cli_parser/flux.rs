use clap::Subcommand;

#[derive(Subcommand)]
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
    }
}