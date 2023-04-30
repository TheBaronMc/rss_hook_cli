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

#[derive(Deserialize)]
pub struct Hook {
    pub sourceId: u64,
    pub destinationId: u64
}

#[derive(Deserialize,Debug)]
pub struct Exception {
    pub statusCode: u64,
    pub message: String
}
