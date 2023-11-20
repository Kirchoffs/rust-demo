use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    headers
        .get("x-custom-header")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}