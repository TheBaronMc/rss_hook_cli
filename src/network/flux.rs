use super::Client;
use super::super::types::{Flux, Exception};
use super::super::utils::processResponse;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};

pub async fn get_all(client: &Client) -> Result<Vec<Flux>, Exception> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    let res = client.get(path, Some(headers), Some(body)).await;
    processResponse::<Vec<Flux>>(res).await
}

pub async fn create(client: &Client, flux_url: String) -> Result<Flux, Exception> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"url\":\"{}\"}}", flux_url);

    let res = client.post(path, Some(headers), Some(body)).await;
    processResponse::<Flux>(res).await
}

pub async fn delete(client: &Client, flux_id: i64) -> Result<Flux, Exception> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":{}}}", flux_id);

    let res = client.delete(path, Some(headers), Some(body)).await;
    processResponse::<Flux>(res).await
}

pub async fn update(client: &Client, flux_id: i64, flux_url: String) -> Result<Flux, Exception> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":{},\"url\":\"{}\"}}", flux_id, flux_url);

    let res = client.patch(path, Some(headers), Some(body)).await;
    processResponse::<Flux>(res).await
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
        let res = get_all(&client).await;

        // Test
        match res {
            Ok(_)   => Err(String::from("The function return something")),
            Err(e)  => Ok(())
        }
    }

    #[tokio::test]
    async fn get_all_empty() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/flux"))
            .respond_with(status_code(200).body("[]")),
        );

        // Run function to test
        let flux = get_all(&client).await?;

        // Test response
        assert!(flux.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_filled_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/flux"))
            .respond_with(status_code(200).body("[ 
                { \"id\": 1, \"url\": \"http://toto.org\"},
                { \"id\": 2, \"url\": \"http://tata.org\"},
                { \"id\": 3, \"url\": \"http://tutu.org\"},
                { \"id\": 4, \"url\": \"http://titi.org\"}
            ]")),
        );

        // Run function to test
        let flux = get_all(&client).await?;

        // Test response
        assert!(flux.len() == 4);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("GET", "/flux"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = get_all(&client).await;

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
    async fn crete_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("POST", "/flux"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"url": "toto"})))),       
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 1, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let flux = create(&client, String::from("toto")).await?;

        // Test response
        assert!(flux.id == 1);
        assert!(flux.url == String::from("http://toto.org"));

        Ok(())
    }

    #[tokio::test]
    async fn create_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("POST", "/flux"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = create(&client, String::from("")).await;

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
    async fn delete_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("DELETE", "/flux"),
                    // Check the content of the body
                    request::body(json_decoded(eq(serde_json::json!({"id": 1})))),       
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 1, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let flux = delete(&client, 1).await?;

        // Test response
        assert!(flux.id == 1);
        assert!(flux.url == String::from("http://toto.org"));

        Ok(())
    }

    #[tokio::test]
    async fn delete_error_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("DELETE", "/flux"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = delete(&client, 1).await;

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
    async fn update_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(
                all_of![
                    request::method_path("PATCH", "/flux"),
                    request::body(json_decoded(eq(serde_json::json!({"id": 1, "url": "toto"}))))
                ]
            ).respond_with(status_code(200).body(" 
                { \"id\": 2, \"url\": \"http://toto.org\"}")),
        );

        // Run function to test
        let flux = update(&client, 1, String::from("toto")).await?;

        // Test response
        assert!(flux.id == 2);
        assert!(flux.url == String::from("http://toto.org"));

        Ok(())
    }

    #[tokio::test]
    async fn get_all_Exception_test() -> Result<(), Exception> {
        // Setup Client and Server
        let server = get_server();
        let client = get_client(Some(&server));

        // Setup available path
        server.expect(
            Expectation::matching(request::method_path("PATCH", "/flux"))
            .respond_with(status_code(200).body("
                { 
                    \"statusCode\": 1,
                    \"message\": \"toto\"
                }")),
        );

        // Run function to test
        let res = update(&client, 1, String::from("toto")).await;

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