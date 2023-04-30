use super::Client;
use super::super::types::Webhook;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};
use reqwest::Error;

pub async fn get_all(client: Client) -> Result<Vec<Webhook>, Error> {
    let path = "/webhooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    client.get(path, Some(headers), Some(body)).await?
    .json::<Vec<Webhook>>().await
}


pub async fn create(client: Client, webhook_url: String) -> Result<Webhook, Error> {
    let path = "/webhooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"url\":\"{}\"}}", webhook_url);

    client.post(path, Some(headers), Some(body)).await?
    .json::<Webhook>().await
}

pub async fn delete(client: Client, webhook_id: i64) -> Result<Webhook, Error> {
    let path = "/webhooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":{}}}", webhook_id);

    client.delete(path, Some(headers), Some(body)).await?
    .json::<Webhook>().await
}

pub async fn update(client: Client, webhook_id: i64, webhook_url: String) -> Result<Webhook, Error> {
    let path = "/webhooks";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":{},\"url\":\"{}\"}}", webhook_id, webhook_url);

    client.patch(path, Some(headers), Some(body)).await?
    .json::<Webhook>().await
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
    async fn get_all_connection_error() -> Result<(), String> {
        // Setup Client
        let client = get_client(None);

        // Run function to test
        let res = get_all(client).await;

        // Test
        match res {
            Ok(_)   => Err(String::from("The function return something")),
            Err(e)  => Ok(())
        }
    }

    #[tokio::test]
    async fn get_all_empty() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/webhooks"))
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let webhooks = get_all(client).await?;

        // Test response
        assert!(webhooks.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_filled_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/webhooks"))
            .respond_with(status_code(200).body("[ 
                { \"id\": 1, \"url\": \"http://toto.org\"},
                { \"id\": 2, \"url\": \"http://tata.org\"},
                { \"id\": 3, \"url\": \"http://tutu.org\"},
                { \"id\": 4, \"url\": \"http://titi.org\"}
            ]")),
        );

        // Run function to test
        let webhooks = get_all(client).await?;

        // Test response
        assert!(webhooks.len() == 4);

        Ok(())
    }

    #[tokio::test]
    async fn crete_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("POST", "/webhooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"url": "toto"})))),       
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 1, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let webhook = create(client, String::from("toto")).await?;

        // Test response
        assert!(webhook.id == 1);
        assert!(webhook.url == String::from("http://toto.org"));

        Ok(())
    }

    #[tokio::test]
    async fn delete_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("DELETE", "/webhooks"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"id": 1})))),       
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 1, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let webhook = delete(client, 1).await?;

        // Test response
        assert!(webhook.id == 1);
        assert!(webhook.url == String::from("http://toto.org"));

        Ok(())
    }

    #[tokio::test]
    async fn update_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("PATCH", "/webhooks"),
                    request::body(json_decoded(eq(serde_json::json!({"id": 1, "url": "toto"}))))
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 2, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let webhook = update(client, 1, String::from("toto")).await?;

        // Test response
        assert!(webhook.id == 2);
        assert!(webhook.url == String::from("http://toto.org"));

        Ok(())
    }
}