mod hello_world;
mod body_string;
mod body_json;
mod path_variables;
mod query_params;
mod user_agent;
mod custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;
mod get_json;
mod make_request;
mod get_cache;

use std::{sync::{Arc, Mutex}, collections::HashMap};

use axum::{Router, routing::get, routing::post, body::Body, http::Method, Extension, middleware, extract::Path};
use hello_world::hello_world;
use body_string::mirror_body_string;
use body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;
use tower_http::cors::{CorsLayer, Any};
use user_agent::mirror_user_agent;
use custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_errors::aways_errors;
use returns_201::returns_201;
use get_json::get_json;
use make_request::make_request;
use get_cache::get_cache;

#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

type GetCache = Arc<Mutex<HashMap<i32, i32>>>;

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from middleware!".to_string()
    };

    let cache: GetCache = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_errors", get(aways_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/make_request", get(make_request))
        .route("/make_request_and_cache/:key", get(move |Path(key): Path<i32>| {
            get_cache(key, cache)
        }))
        .layer(Extension(shared_data))
        .layer(cors)
}