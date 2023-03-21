mod flux;
mod webhooks;
mod hooks;
mod articles;
mod deliveries;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "Kevin K. <kbknapp@gmail.com>")]
#[command(version = "1.0")]
pub struct CliParser {
    #[command(subcommand)]
    commands: Commands,
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