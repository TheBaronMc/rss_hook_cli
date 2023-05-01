pub mod flux;
pub mod webhook;
pub mod article;
pub mod hooks;
pub mod deliveries;

use reqwest::{Client as ReqwestClient, RequestBuilder};
use reqwest::header::HeaderMap;
use reqwest::{Response, Error};

pub struct Client {
    url: String,
    http_client: ReqwestClient
}

impl Client {
    pub fn new(address: String, port: u64) -> Client {
        Client {
            url: format!("http://{}:{}", address, port),
            http_client: ReqwestClient::new()
        }
    }
    
    pub async fn get(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .get(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    pub async fn post(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .post(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    pub async fn patch(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .patch(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    pub async fn delete(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .delete(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }
}

async fn send(mut request: RequestBuilder, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
    if let Some(h) = headers {
        request = request.headers(h);
    }

    if let Some(b) = body {
        request = request.body(b);
    }

    request.send().await
}
