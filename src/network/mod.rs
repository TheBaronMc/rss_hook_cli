mod flux;

use reqwest::{Client as ReqwestClient, RequestBuilder};
use reqwest::header::HeaderMap;
use reqwest::{Response, Error};
use std::collections::HashMap;

pub struct Client {
    url: String,
    http_client: ReqwestClient
}

impl Client {
    fn new(address: String, port: u64) -> Client {
        Client {
            url: format!("http://{}:{}", address, port),
            http_client: ReqwestClient::new()
        }
    }
    
    async fn get(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .get(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    async fn post(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .post(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    async fn patch(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .patch(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }

    async fn delete(&self, path: &str, headers: Option<HeaderMap>, body: Option<String>) -> Result<Response, Error> {
        let request = self.http_client
        .delete(format!("{}{}", self.url, path));

        send(request, headers, body).await
    }
}

fn args_to_query_string(args: HashMap<String,String>) -> String {
    let mut path= String::new();

    path += "?";
    let nb_keys = args.keys().len();
    let mut index = 0;

    for (key, value) in args {
        path += format!("{}={}", key, value).as_str();
        if index < nb_keys-1 {
            path += "&";
        }
        index += 1;
    }

    path
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
