use anyhow::Result;
use axum::http::{HeaderMap, HeaderValue};
use axum::{extract::Path, routing::get, http::StatusCode, Router, Extension};
use image::ImageOutputFormat;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use std::convert::TryInto;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use percent_encoding::{percent_decode_str, percent_encode, AsciiSet, NON_ALPHANUMERIC, CONTROLS};
use bytes::Bytes;
use lru::LruCache;
use tracing::{info, instrument};

mod pb;
use pb::{Filter, Resize, Spec, ImageSpec};
use pb::{resize, filter};

mod engine;
use engine::Photon;
use engine::{Engine, SpecTransform};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new().layer(Extension(cache)));

    let addr = "127.0.0.1:3000".parse().unwrap();
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>, Extension(cache): Extension<Cache>) 
    -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let url: &str = &percent_decode_str(&url).decode_utf8_lossy();
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));
    info!("Finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

#[test]
fn print_test_url() {
    use std::borrow::Borrow;
    let url = "https://images.pexels.com/photos/14361428/pexels-photo-14361428.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2";
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    const GENERAL_URL: &AsciiSet = &CONTROLS.add(b' ').add(b'/').add(b'?').add(b'#').add(b'=').add(b'&').add(b'%');
    let test_image = percent_encode(url.as_bytes(), GENERAL_URL).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
