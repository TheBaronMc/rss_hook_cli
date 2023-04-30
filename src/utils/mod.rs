use std::borrow::Borrow;

use crate::types::Exception;
use httptest::http::response;
use reqwest::{Response, Error};
use serde::de::DeserializeOwned;

pub async fn parseBody<T: DeserializeOwned>(response: Response) -> Result<T, Exception> {
    match response.text().await {
        Ok(body) =>  {
            if let Ok(o) = serde_json::from_str::<T>(body.as_str()) {
                return Ok(o);
            }

            if let Ok(o) = serde_json::from_str::<Exception>(body.as_str()) {
                return Err(o);
            }

            panic!("Unkown body type")
        },
        Err(_)  => panic!("Error during body extraction")
    }
}

pub async fn processResponse<T: DeserializeOwned>(result: Result<Response,Error>) -> Result<T, Exception> {
    match result {
        Ok(response) => parseBody::<T>(response).await,
        Err(error) => {
            let mut status_code = 500;

            if let Some(code) = error.status() {
                status_code = code.as_u16() as u64;
            }

            Err(Exception { statusCode: status_code, message: error.to_string() })
        }
    }
}