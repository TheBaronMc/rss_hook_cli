pub mod flux;
pub mod webhooks;
pub mod hooks;
pub mod articles;
pub mod deliveries;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RSS Hook CLI")]
#[command(author = "Charly G.")]
#[command(version = "1.0")]
pub struct CliParser {
    #[command(subcommand)]
    pub commands: Commands,
    #[arg(short, long, default_value = "localhost")]
    pub server: Option<String>,
    #[arg(short, long, default_value = "3000")]
    pub port: Option<i64>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Actions related to RSS Flux (add, del, update)
    Flux {
        #[command(subcommand)]
        commands: flux::Commands 
    },
    /// Actions related to webhooks (add, del, update)
    Webhooks {
        #[command(subcommand)]
        commands: webhooks::Commands
    },
    /// Actions related to articles (ls)
    Articles {
        #[command(subcommand)]
        commands: articles::Commands
    },
    /// Actions related to hooks (create, ls, del)
    Hooks {
        #[command(subcommand)]
        commands: hooks::Commands
    },
    /// Actions related deliveries (ls)
    Deliveries {
        #[command(subcommand)]
        commands: deliveries::Commands
    }
}