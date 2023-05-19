use clap::Subcommand;
use super::BANNER;

#[derive(Subcommand)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
pub enum Commands {
    /// List all articles
    Ls {
        // Filter on flux id
        #[arg(short, long)]
        flux_id: Option<u64>
    },
    // List all webhooks which received the article
    Deliveries {
        #[arg(short, long)]
        article_id: u64
    }
}