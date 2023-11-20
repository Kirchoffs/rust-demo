use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct MirrorJson {
    message: String
}

#[derive(Serialize)]
pub struct MirrorJsonResp {
    message: String,
    message_from_server: String
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResp> {
    Json(
        MirrorJsonResp {
            message: body.message,
            message_from_server: "Hello, World!".to_owned()
        }
    )
}