mod cli_parser;
mod network;
mod types;
mod utils;

use clap::Parser;

use cli_parser::CliParser;

fn main() {
    let _args = CliParser::parse();
}
