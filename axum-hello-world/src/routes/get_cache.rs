use std::{sync::{Arc, Mutex}, collections::HashMap};
use axum::{response::IntoResponse, Json};
use rand::Rng;

type GetCache = Arc<Mutex<HashMap<i32, i32>>>;

pub async fn get_cache(key: i32, cache: GetCache) -> impl IntoResponse {
    println!("Received key: {}", key);
    let mut cache = cache.lock().unwrap();

    let res = match cache.get(&key) {
        Some(value) => {
            println!("Found value in cache: {}", value);
            value.to_owned()
        },
        None => {
            let mut rng = rand::thread_rng();
            let value = rng.gen::<i32>();
            cache.insert(key, value);
            println!("Generated new value: {}", value);
            value
        }
    };

    Json(res).into_response()
}
