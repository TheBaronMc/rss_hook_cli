mod cli_parser;
mod network;
mod types;
mod utils;

use clap::Parser;

use cli_parser::CliParser;

#[tokio::main]
async fn main() -> Result<(), types::Exception>{
    let args = CliParser::parse();

    let mut server = String::from("localhost");
    let mut port = 3000;

    if let Some(s) = args.server {
        server = s;
    }

    if let Some(p) = args.port {
        port = p;
    }

    let client = network::Client::new(server, port as u64);

    match args.commands {
        cli_parser::Commands::Flux { commands } => {
            match commands {
                cli_parser::flux::Commands::Add { flux_url } => {
                    let res = network::flux::create(&client, flux_url).await;

                    match res {
                        Ok(flux) => println!("Created with id {}", flux.id),
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
                            for i in 0..flux.len() {
                                println!("{} - id: {}, url: {}", i, flux[i].id, flux[i].url);
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
                        Ok(webhook) => println!("Created with id {}", webhook.id),
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
                cli_parser::webhooks::Commands::List {  } => {
                    let res = network::webhook::get_all(&client).await;

                    match res {
                        Ok(webhooks) => {
                            for i in 0..webhooks.len() {
                                println!("{} - id: {}, url: {}", i, webhooks[i].id, webhooks[i].url);
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
                            for i in 0..articles.len() {
                                println!("{} - id: {}, url: {}, source: {}", 
                                i, 
                                articles[i].id, 
                                if let Some(url) = articles[i].url.clone() { url } else { "".to_string()},
                                articles[i].sourceId);
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
                },
                cli_parser::hooks::Commands::Ls { flux_id, webhook_id } => {
                    if let Some(id) = flux_id {
                        let response = network::hooks::get_all_bind_to_flux(&client, id).await;

                        match response {
                            Ok(flux) => {
                                for i in 0..flux.len() {
                                    println!("{} - id: {}, url: {}", i, flux[i].id, flux[i].url);
                                }
                            },
                            Err(e) => return Err(e)
                        }
                    }
                    
                    if let Some(id) = webhook_id {
                        let response = network::hooks::get_all_bind_to_webhook(&client, id).await;

                        match response {
                            Ok(webhooks) => {
                                for i in 0..webhooks.len() {
                                    println!("{} - id: {}, url: {}", i, webhooks[i].id, webhooks[i].url);
                                }
                            },
                            Err(e) => return Err(e)
                        }
                    }
                }
            }
        },
        cli_parser::Commands::Deliveries { commands } => {
            match commands {
                cli_parser::deliveries::Commands::Ls { article_id, webhook_id } => {
                    if let Some(id) = webhook_id {
                        let response = network::deliveries::get_all_received(&client, id).await;

                        match response {
                            Ok(articles) => {
                                for i in 0..articles.len() {
                                    println!("{} - id: {}, url: {}, source: {}", 
                                    i, 
                                    articles[i].id, 
                                    if let Some(url) = articles[i].url.clone() { url } else { "".to_string()},
                                    articles[i].sourceId);
                                }
                            },
                            Err(e) => return Err(e)
                        }
                    }

                    if let Some(id) = article_id {
                        let response = network::deliveries::get_all_receiver(&client, id).await;

                        match response {
                            Ok(webhooks) => {
                                for i in 0..webhooks.len() {
                                    println!("{} - id: {}, url: {}", i, webhooks[i].id, webhooks[i].url);
                                }
                            },
                            Err(e) => return Err(e)
                        }
                    }
                }
            }
        },
    }

    Ok(())
}
