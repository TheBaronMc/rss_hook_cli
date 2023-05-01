use super::Client;
use super::super::types::{Article,Webhook,Exception};
use super::super::utils::process_response;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};

pub async fn get_all_receiver(client: &Client, article_id: u64) -> Result<Vec<Webhook>, Exception> {
    let path = format!("/deliveries/articles?id={}", article_id);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client.get(path.as_str(), Some(headers), None).await;
    process_response(res).await
}

pub async fn get_all_received(client: &Client, webhook_id: u64) -> Result<Vec<Article>, Exception> {
    let path = format!("/deliveries/webhooks?id={}", webhook_id);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client.get(path.as_str(), Some(headers), None).await;
    process_response(res).await
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
    async fn get_all_receiver_empty() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(                
                all_of![
                    request::method_path("GET", "/deliveries/articles"),
                    request::query(url_decoded(contains(("id", "1")))),
                ]
            )
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let webhooks = get_all_receiver(&client, 1).await?;

        // Test response
        assert!(webhooks.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_receiver_filled() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/deliveries/articles"),
                    request::query(url_decoded(contains(("id", "1")))),
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
        let webhooks = get_all_receiver(&client, 1).await?;

        // Test response
        assert!(webhooks.len() == 4);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_receiver_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/deliveries/articles"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = get_all_receiver(&client, 1).await;

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
    async fn get_all_received_empty() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(                
                all_of![
                    request::method_path("GET", "/deliveries/webhooks"),
                    request::query(url_decoded(contains(("id", "1")))),
                ]
            )
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let articles = get_all_received(&client, 1).await?;

        // Test response
        assert!(articles.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_received_filled() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/deliveries/webhooks"),
                    request::query(url_decoded(contains(("id", "1")))),
                ]
            )
            .respond_with(status_code(200).body("[ 
                { 
                    \"id\": 1,
                    \"title\": \"toto\",
                    \"pub_date\": \"2023-04-23T18:47:42.531Z\",
                    \"description\": \"Great Article\",
                    \"url\": \"http://toto.org\",
                    \"sourceId\": 1
                },
                { 
                    \"id\": 2,
                    \"title\": \"tata\", 
                    \"pub_date\": \"2023-04-23T18:47:42.531Z\", 
                    \"description\": \"Great Article\", 
                    \"sourceId\": 1
                }
            ]")),
        );

        // Run function to test
        let articles = get_all_received(&client, 1).await?;

        // Test response
        assert!(articles.len() == 2);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_received_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/deliveries/webhooks"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = get_all_received(&client, 1).await;

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