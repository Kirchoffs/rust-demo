#![warn(clippy::all)]

mod routes;
mod types;
mod profanity;
mod store;

use std::env;

use config::Config;
use routes::{
    answer::add_answer, 
    authenticate::{auth, login, register}, 
    question::{add_question, delete_question, get_questions, update_question}
};

use serde::Deserialize;
use tracing::event;
use tracing_subscriber::fmt::format::FmtSpan;
use types::question::QuestionId;
use store::Store;

use warp::{
    http::Method,
    Filter,
};

use handle_errors::return_error;

#[derive(Debug, Default, Deserialize)]
struct Args {
    log_level: String,
    database_host: String,
    database_port: u16,
    database_name: String,
}

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();
    
    if let Err(_) = env::var("BAD_WORDS_API_KEY") {
        panic!("BAD_WORDS_API_KEY is not set");
    }

    if let Err(_) = env::var("PASETO_KEY") {
        panic!("PASETO_KEY is not set");
    }

    let port = env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(6174))
        .map_err(|err| handle_errors::Error::ParseError(err))?;

    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();

    let config = config.try_deserialize::<Args>().unwrap();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_|
        format!(
            "handle_errors={},qa_web_service={},warp={}",
            config.log_level,
            config.log_level,
            config.log_level
        )
    );

    let store = Store::new(&format!(
        "postgres://{}:{}/{}",
        config.database_host, config.database_port, config.database_name
    ))
    .await
    .map_err(|err| handle_errors::Error::DatabaseQueryError(err))?;

    sqlx::migrate!()
        .run(&store.connection)
        .await
        .map_err(|err| handle_errors::Error::MigrationError(err))?;

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and(auth())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and(auth())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(auth())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(store_filter.clone())
        .and(auth())
        .and_then(add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(login);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "incoming request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4().to_string()
            ) 
        }))
        .recover(return_error);

    event!(tracing::Level::INFO, "QA web service is starting with build ID: {}", env!("QA_WEB_VERSION"));
    event!(tracing::Level::INFO, "server is going to start");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
