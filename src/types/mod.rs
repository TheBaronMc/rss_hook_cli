use serde::Deserialize;

#[derive(Deserialize)]
pub struct Flux {
    pub id: u64,
    pub url: String
}

#[derive(Deserialize)]
pub struct Webhook {
    pub id: u64,
    pub url: String
}

#[derive(Deserialize)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub pub_date: String,
    pub url: Option<String>,
    pub sourceId: u64
}