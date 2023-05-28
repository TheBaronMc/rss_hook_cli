pub mod printer;
pub mod file;
pub mod csv;

use crate::types::Exception;
use reqwest::{Response, Error};
use serde::de::DeserializeOwned;

pub async fn parse_body<T: DeserializeOwned>(response: Response) -> Result<T, Exception> {
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

pub async fn process_response<T: DeserializeOwned>(result: Result<Response,Error>) -> Result<T, Exception> {
    match result {
        Ok(response) => parse_body::<T>(response).await,
        Err(error) => {
            let mut status_code = 500;

            if let Some(code) = error.status() {
                status_code = code.as_u16() as u64;
            }

            Err(Exception { statusCode: status_code, message: error.to_string() })
        }
    }
}

pub fn count_digits(num: u64) -> u64 {
    if num > 0 {
        1 + count_digits(num.div_euclid(10))
    } else {
        0
    }
}