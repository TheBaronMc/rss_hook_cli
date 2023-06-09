use super::Client;
use super::super::types::{Flux,Webhook,Hook,Exception};
use super::super::utils::process_response;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};

pub async fn get_all_bind_to_flux(client: &Client, flux_id: u64) -> Result<Vec<Webhook>, Exception> {
    let path = format!("/hooks/flux?id={}", flux_id);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    let res = client.get(path.as_str(), Some(headers), Some(body)).await;
    process_response(res).await
}

pub async fn get_all_bind_to_webhook(client: &Client, webhook_id: u64) -> Result<Vec<Flux>, Exception> {
    let path = format!("/hooks/webhook?id={}", webhook_id);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    let res = client.get(path.as_str(), Some(headers), Some(body)).await;
    process_response(res).await
}

pub async fn create(client: &Client, hook: Hook) -> Result<bool, Exception> {
    let path = "/hooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"flux_id\": {}, \"webhook_id\": {} }}", hook.sourceId, hook.destinationId);

    let res = client.post(path, Some(headers), Some(body)).await;
    process_response::<bool>(res).await
}

pub async fn delete(client: &Client, hook: Hook) -> Result<bool, Exception> {
    let path = "/hooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"flux_id\": {}, \"webhook_id\": {}}}", hook.sourceId, hook.destinationId);

    let res = client.delete(path, Some(headers), Some(body)).await;
    process_response::<bool>(res).await
}

#[cfg(test)]
mod tests {
    use httptest::{Server, Expectation, matchers::*, responders::*};
    use super::*;
    use super::super::Client;

    fn get_server() -> Server {
        Server::run()
    }

    fn get_client(server: Option<&Server>) -> Client {
        match server {
            Some(server) => {
                Client::new(
                    String::from("localhost"),
                    server.addr().port() as u64
                )
            },
            None => {
                Client::new(String::from("uncorrect_domain"), 80)
            }
        }
    }

    #[tokio::test]
    async fn get_all_bind_to_flux_empty() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/hooks/flux"),
                    request::query(url_decoded(contains(("id", "1"))))
                ]
            )
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let webhooks = get_all_bind_to_flux(&client, 1).await?;

        // Test response
        assert!(webhooks.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_bind_to_flux_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/hooks/flux"),
                    request::query(url_decoded(contains(("id", "1"))))
                ]
            )
            .respond_with(status_code(200).body("[ 
                { \"id\": 1, \"url\": \"http://toto.org\"},
                { \"id\": 2, \"url\": \"http://tata.org\"},
                { \"id\": 3, \"url\": \"http://tutu.org\"},
                { \"id\": 4, \"url\": \"http://titi.org\"}
            ]")),
        );

        // Run function to test
        let webhooks = get_all_bind_to_flux(&client, 1).await?;

        // Test response
        assert!(webhooks.len() == 4);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_bind_to_flux_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/hooks/flux"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = get_all_bind_to_flux(&client, 1).await;

        // Test response
        match res {
            Ok(_) => Err(Exception { statusCode: 1, message: String::from("No exception") }),
            Err(e) => {
                if e.message == String::from("toto") && e.statusCode == 1 {
                    Ok(())
                } else {
                    Err(Exception { statusCode: 1, message: String::from("Wrong values") })
                }
            }
        }
    }

    #[tokio::test]
    async fn get_all_bind_to_webhook_empty() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/hooks/webhook"),
                    request::query(url_decoded(contains(("id", "1"))))
                ]
            )
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let flux = get_all_bind_to_webhook(&client, 1).await?;

        // Test response
        assert!(flux.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_bind_to_webhook_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/hooks/webhook"),
                    request::query(url_decoded(contains(("id", "1"))))
                ]
            )
            .respond_with(status_code(200).body("[ 
                { \"id\": 1, \"url\": \"http://toto.org\"},
                { \"id\": 2, \"url\": \"http://tata.org\"},
                { \"id\": 3, \"url\": \"http://tutu.org\"},
                { \"id\": 4, \"url\": \"http://titi.org\"}
            ]")),
        );

        // Run function to test
        let flux = get_all_bind_to_webhook(&client, 1).await?;

        // Test response
        assert!(flux.len() == 4);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_bind_to_webhook_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/hooks/webhook"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = get_all_bind_to_webhook(&client, 1).await;

        // Test response
        match res {
            Ok(_) => Err(Exception { statusCode: 1, message: String::from("No exception") }),
            Err(e) => {
                if e.message == String::from("toto") && e.statusCode == 1 {
                    Ok(())
                } else {
                    Err(Exception { statusCode: 1, message: String::from("Wrong values") })
                }
            }
        }
    }

    #[tokio::test]
    async fn create_success_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("POST", "/hooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"flux_id": 1, "webhook_id": 2})))),       
                ]
            ).respond_with(status_code(200).body("true")),
        );

        // Run function to test
        let created = create(
            &client,
            Hook { sourceId: 1, destinationId: 2 }).await?;

        // Test response
        assert!(created);

        Ok(())
    }

    #[tokio::test]
    async fn create_fail_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("POST", "/hooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"flux_id": 1, "webhook_id": 2})))),       
                ]
            ).respond_with(status_code(200).body("false")),
        );

        // Run function to test
        let created = create(
            &client,
            Hook { sourceId: 1, destinationId: 2 }).await?;

        // Test response
        assert!(!created);

        Ok(())
    }

    #[tokio::test]
    async fn create_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("POST", "/hooks"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = create(&client, Hook { sourceId: 1, destinationId: 1 }).await;

        // Test response
        match res {
            Ok(_) => Err(Exception { statusCode: 1, message: String::from("No exception") }),
            Err(e) => {
                if e.message == String::from("toto") && e.statusCode == 1 {
                    Ok(())
                } else {
                    Err(Exception { statusCode: 1, message: String::from("Wrong values") })
                }
            }
        }
    }

    #[tokio::test]
    async fn delete_success_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("DELETE", "/hooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"flux_id": 1, "webhook_id": 2})))),       
                ]
            ).respond_with(status_code(200).body("true")),
        );

        // Run function to test
        let created = delete(
            &client,
            Hook { sourceId: 1, destinationId: 2 }).await?;

        // Test response
        assert!(created);

        Ok(())
    }

    #[tokio::test]
    async fn delete_fail_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("DELETE", "/hooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"flux_id": 1, "webhook_id": 2})))),       
                ]
            ).respond_with(status_code(200).body("false")),
        );

        // Run function to test
        let created = delete(
            &client,
            Hook { sourceId: 1, destinationId: 2 }).await?;

        // Test response
        assert!(!created);

        Ok(())
    }

    #[tokio::test]
    async fn delete_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("DELETE", "/hooks"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = delete(&client, Hook { sourceId: 1, destinationId: 1 }).await;

        // Test response
        match res {
            Ok(_) => Err(Exception { statusCode: 1, message: String::from("No exception") }),
            Err(e) => {
                if e.message == String::from("toto") && e.statusCode == 1 {
                    Ok(())
                } else {
                    Err(Exception { statusCode: 1, message: String::from("Wrong values") })
                }
            }
        }
    }


}