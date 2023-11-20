use axum::http::StatusCode;

pub async fn aways_errors() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}