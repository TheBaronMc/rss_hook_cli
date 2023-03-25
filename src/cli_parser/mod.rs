mod flux;
mod webhooks;
mod hooks;
mod articles;
mod deliveries;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RSS Hook CLI")]
#[command(author = "Charly G.")]
#[command(version = "1.0")]
pub struct CliParser {
    #[command(subcommand)]
    commands: Commands,
    #[arg(short, long, default_value = "localhost")]
    server: Option<String>,
    #[arg(short, long, default_value = "3000")]
    port: Option<i64>
}

#[derive(Subcommand)]
enum Commands {
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