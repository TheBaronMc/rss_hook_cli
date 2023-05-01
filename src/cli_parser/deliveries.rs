use clap::Subcommand;
use super::BANNER;

#[derive(Subcommand)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
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