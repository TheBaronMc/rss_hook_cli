use serde::Deserialize;

#[derive(Deserialize)]
pub struct Flux {
    pub id: u64,
    pub url: String
}
