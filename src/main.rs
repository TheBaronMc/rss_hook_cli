mod cli_parser;

use clap::Parser;

use cli_parser::CliParser;

fn main() {
    let _args = CliParser::parse();
}
