use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// List all articles
    Ls {
        // Filter on flux id
        #[arg(short, long)]
        flux_id: Option<u64>
    },
}