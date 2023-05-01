pub mod flux;
pub mod webhooks;
pub mod hooks;
pub mod articles;
pub mod deliveries;

use clap::{Parser, Subcommand};

// From https://patorjk.com/software/taag/#p=display&f=Big%20Money-ne&t=RssHookCli
pub const BANNER: &str = r"
//$$$$$$$                     /$$   /$$                     /$$        /$$$$$$  /$$ /$$
| $$__  $$                   | $$  | $$                    | $$       /$$__  $$| $$|__/
| $$  \ $$  /$$$$$$$ /$$$$$$$| $$  | $$  /$$$$$$   /$$$$$$ | $$   /$$| $$  \__/| $$ /$$
| $$$$$$$/ /$$_____//$$_____/| $$$$$$$$ /$$__  $$ /$$__  $$| $$  /$$/| $$      | $$| $$
| $$__  $$|  $$$$$$|  $$$$$$ | $$__  $$| $$  \ $$| $$  \ $$| $$$$$$/ | $$      | $$| $$
| $$  \ $$ \____  $$\____  $$| $$  | $$| $$  | $$| $$  | $$| $$_  $$ | $$    $$| $$| $$
| $$  | $$ /$$$$$$$//$$$$$$$/| $$  | $$|  $$$$$$/|  $$$$$$/| $$ \  $$|  $$$$$$/| $$| $$
|__/  |__/|_______/|_______/ |__/  |__/ \______/  \______/ |__/  \__/ \______/ |__/|__/
";

#[derive(Parser)]
#[clap(name = "Rss Hook CLI", version = "1.0", author = "Charly G.",
       before_help = BANNER)]
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