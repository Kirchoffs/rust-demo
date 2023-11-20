use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    usename: String,
    id: i32,
}

pub async fn get_json() -> Json<Data> {
    Json(Data {
        message: "test-message".to_string(),
        usename: "test-usename".to_string(),
        id: 42,
    })
}
