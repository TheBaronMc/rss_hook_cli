mod cli_parser;
mod network;
mod types;
mod utils;

use clap::Parser;

use cli_parser::CliParser;

use utils::printer::*;
use utils::printer::article::ArticleFormatter;
use utils::printer::webhook::WebhookFormatter;
use utils::printer::flux::FluxFormatter;

#[tokio::main]
async fn main() -> Result<(), types::Exception>{
    let args = CliParser::parse();

    let output = !args.no_output;

    // Server connection
    let mut server = String::from("localhost");
    let mut port = 3000;

    if let Some(s) = args.server {
        server = s;
    }

    if let Some(p) = args.port {
        port = p;
    }

    let client = network::Client::new(server, port as u64);

    // Formatter
    let formatter_pref = FormatterPref {
        max_str_len: 50,
        column_sep: "|",
        section_sep: "=",
    };

    match args.commands {
        cli_parser::Commands::Flux { commands } => {
            match commands {
                cli_parser::flux::Commands::Add { flux_url } => {
                    let res = network::flux::create(&client, flux_url).await;

                    match res {
                        Ok(flux) => if output { println!("{}", flux.id) },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::flux::Commands::Del { flux_id } => {
                    let res = network::flux::delete(&client, flux_id as i64).await;

                    if let Err(e) = res {
                        return Err(e);
                    }
                },
                cli_parser::flux::Commands::Update { flux_id, flux_url } => {
                    let res = network::flux::update(&client, flux_id as i64, flux_url).await;

                    if let Err(e) = res {
                        return Err(e);
                    }
                },
                cli_parser::flux::Commands::Ls {  } => {
                    let res = network::flux::get_all(&client).await;

                    match res {
                        Ok(flux) => {
                            if output {
                                print(Box::new(FluxFormatter::new(Box::new(flux), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::flux::Commands::Hooks { flux_id } => {
                    let response = network::hooks::get_all_bind_to_flux(&client, flux_id).await;

                    match response {
                        Ok(webhooks) => {
                            if output {
                                print(Box::new(WebhookFormatter::new(Box::new(webhooks), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
        },
        cli_parser::Commands::Webhooks { commands } => {
            match commands {
                cli_parser::webhooks::Commands::Add { webhook_url } => {
                    let res = network::webhook::create(&client, webhook_url).await;

                    match res {
                        Ok(webhook) => if output { println!("{}", webhook.id) },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::webhooks::Commands::Del { webhook_id } => {
                    let res = network::webhook::delete(&client, webhook_id as i64).await;

                    if let Err(e) = res {
                        return Err(e);
                    }
                },
                cli_parser::webhooks::Commands::Update { webhook_id, webhook_url } => {
                    let res = network::webhook::update(&client, webhook_id as i64, webhook_url).await;

                    if let Err(e) = res {
                        return Err(e);
                    }
                },
                cli_parser::webhooks::Commands::Ls {  } => {
                    let res = network::webhook::get_all(&client).await;

                    match res {
                        Ok(webhooks) => {
                            if output {
                                print(Box::new(WebhookFormatter::new(Box::new(webhooks), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::webhooks::Commands::Deliveries { webhook_id } => {
                    let response = network::deliveries::get_all_received(&client, webhook_id).await;

                    match response {
                        Ok(articles) => {
                            if output {
                                print(Box::new(ArticleFormatter::new(Box::new(articles), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::webhooks::Commands::Hooks { webhook_id } => {
                    let response = network::hooks::get_all_bind_to_webhook(&client, webhook_id).await;

                    match response {
                        Ok(flux) => {
                            if output {
                                print(Box::new(FluxFormatter::new(Box::new(flux), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
        },
        cli_parser::Commands::Articles { commands } => {
            match commands {
                cli_parser::articles::Commands::Ls { flux_id } => {
                    let response: Result<Vec<types::Article>,types::Exception>;

                    match flux_id {
                        Some(id) => response = network::article::get_all_from(&client, id).await,
                        None => response = network::article::get_all(&client).await
                    }

                    match response {
                        Ok(articles) => {
                            if output {
                                print(Box::new(ArticleFormatter::new(Box::new(articles), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                },
                cli_parser::articles::Commands::Deliveries { article_id } => {
                    let response = network::deliveries::get_all_receiver(&client, article_id).await;

                    match response {
                        Ok(webhooks) => {
                            if output {
                                print(Box::new(WebhookFormatter::new(Box::new(webhooks), formatter_pref)));
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
        },
        cli_parser::Commands::Hooks { commands } => {
            match commands {
                cli_parser::hooks::Commands::Create { flux_id, webhook_id } => {
                    let response = network::hooks::create(&client, types::Hook { sourceId: flux_id, destinationId: webhook_id }).await;

                    if let Err(e) = response {
                        return Err(e);
                    }
                },
                cli_parser::hooks::Commands::Del { flux_id, webhook_id } => {
                    let response = network::hooks::delete(&client, types::Hook { sourceId: flux_id, destinationId: webhook_id }).await;

                    if let Err(e) = response {
                        return Err(e);
                    }
                }
            }
        },
    }

    Ok(())
}
