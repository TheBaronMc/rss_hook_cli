use super::Client;

use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};
use reqwest::Error;
use std::collections::HashMap;

pub async fn get_all(client: Client) -> Result<HashMap<String, String>, Error> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = String::from("");

    client.get(path, Some(headers), Some(body)).await?
    .json::<HashMap<String, String>>().await
}


pub async fn create(client: Client, flux_url: String) -> Result<HashMap<String, String>, Error> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"url\":\"{}\"}}", flux_url);

    client.post(path, Some(headers), Some(body)).await?
    .json::<HashMap<String, String>>().await
}

pub async fn delete(client: Client, flux_id: i64) -> Result<HashMap<String, String>, Error> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":\"{}\"}}", flux_id);

    client.delete(path, Some(headers), Some(body)).await?
    .json::<HashMap<String, String>>().await
}

pub async fn update(client: Client, flux_id: i64, flux_url: String) -> Result<HashMap<String, String>, Error> {
    let path = "/flux";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = format!("{{\"id\":\"{}\",\"url\":\"{}\"}}", flux_id, flux_url);

    client.patch(path, Some(headers), Some(body)).await?
    .json::<HashMap<String, String>>().await
}