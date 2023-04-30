use super::Client;
use super::super::types::Article;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};
use reqwest::Error;

pub async fn get_all(client: Client) -> Result<Vec<Article>, Error> {
    let path = "/articles";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    client.get(path, Some(headers), Some(body)).await?
    .json::<Vec<Article>>().await
}

pub async fn get_all_from(client: Client, flux_id: u64) -> Result<Vec<Article>, Error> {
    let path = format!("/articles/flux?id={}", flux_id);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    client.get(path.as_str(), Some(headers), Some(body)).await?
    .json::<Vec<Article>>().await
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
            Expectation::matching(request::method_path("GET", "/articles"))
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let articles = get_all(client).await?;

        // Test response
        assert!(articles.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_filled_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/articles"))
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
        let articles = get_all(client).await?;

        // Test response
        assert!(articles.len() == 2);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_from_empty() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/articles/flux"),
                    request::query(url_decoded(contains(("id", "1")))),
                ]
            )
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let articles = get_all_from(client, 1).await?;

        // Test response
        assert!(articles.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_from_filled_test() -> Result<(), Error> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("GET", "/articles/flux"),
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
        let articles = get_all_from(client, 1).await?;

        // Test response
        assert!(articles.len() == 2);

        Ok(())
    }
}