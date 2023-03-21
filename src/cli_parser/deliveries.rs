use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// List all deliveries
    Ls {
        /// Filter on article id
        #[arg(short, long)]
        article_id: Option<u64>,
        /// Filter on webhook id
        #[arg(short, long)]
        webhook_id: Option<u64>
    }
}