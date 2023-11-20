use std::collections::HashMap;

use reqwest::StatusCode;

pub async fn make_request() -> Result<String, StatusCode> {
    match reqwest::get("https://www.rust-lang.org/").await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    println!("body: {}", body);
                    Ok(body)
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
