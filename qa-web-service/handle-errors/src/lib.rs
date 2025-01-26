use std::fmt;

use tracing::{event, instrument, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden}, http::StatusCode, reject::{Reject, Rejection}, reply::Reply
};
use argon2::Error as ArgonError;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    CannotVerifyToken,
    Unauthorized,
    ArgonLibraryError(ArgonError),
    DatabaseQueryError(sqlx::Error),
    MigrationError(sqlx::migrate::MigrateError),
    ReqwestAPIError(reqwest::Error),
    MiddlewareReqwestAPIError(reqwest_middleware::Error),
    ClientError(APILayerError),
    ServerError(APILayerError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(err) => write!(formatter, "parse error: {}", err),
            Error::MissingParameters => write!(formatter, "missing parameters"),
            Error::WrongPassword => write!(formatter, "wrong password"),
            Error::CannotVerifyToken => write!(formatter, "cannot verify token"),
            Error::Unauthorized => write!(formatter, "unauthorized"),
            Error::ArgonLibraryError(err) => write!(formatter, "argon2 library error: {}", err),
            Error::DatabaseQueryError(_) => write!(formatter, "database query error"),
            Error::MigrationError(err) => write!(formatter, "migration error: {}", err),
            Error::ReqwestAPIError(err) => write!(formatter, "external API error: {}", err),
            Error::MiddlewareReqwestAPIError(err) => write!(formatter, "external API error: {}", err),
            Error::ClientError(err) => write!(formatter, "client error: {}", err),
            Error::ServerError(err) => write!(formatter, "server error: {}", err),
        }
    }
}

impl Reject for Error {}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl fmt::Display for APILayerError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "status: {}, message: {}", self.status, self.message)
    }
}

impl Reject for APILayerError {}

const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = rejection.find() {
        event!(Level::ERROR, "database query error");
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status("duplicate key".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
                } else {
                    Ok(warp::reply::with_status("internal server error".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
                }
            },
            _ => {
                Ok(warp::reply::with_status("internal server error".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
            }
        }
    } else if let Some(Error::ReqwestAPIError(err)) = rejection.find() {
        event!(Level::ERROR, "external API error: {}", err);
        Ok(warp::reply::with_status(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(Error::Unauthorized) = rejection.find() {
        event!(Level::ERROR, "unauthorized");
        Ok(warp::reply::with_status("no permission".to_string(), StatusCode::UNAUTHORIZED))
    } else if let Some(Error::WrongPassword) = rejection.find() {
        event!(Level::ERROR, "wrong password");
        Ok(warp::reply::with_status("wrong password".to_string(), StatusCode::UNAUTHORIZED))
    } else if let Some(Error::MiddlewareReqwestAPIError(err)) = rejection.find() {
        event!(Level::ERROR, "external API error: {}", err);
        Ok(warp::reply::with_status(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(Error::ClientError(err)) = rejection.find() {
        event!(Level::ERROR, "client error: {}", err);
        Ok(warp::reply::with_status("internal server error".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(Error::ServerError(err)) = rejection.find() {
        event!(Level::ERROR, "server error: {}", err);
        Ok(warp::reply::with_status("internal server error".to_string(), StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(error) = rejection.find::<Error>() {
        event!(Level::ERROR, "custom error: {}", error);
        Ok(warp::reply::with_status(error.to_string(), StatusCode::BAD_REQUEST))
    } else if let Some(error) = rejection.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS error: {}", error);
        Ok(warp::reply::with_status(error.to_string(), StatusCode::RANGE_NOT_SATISFIABLE))
    } else if let Some(error) = rejection.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "body deserialize error: {}", error);
        Ok(warp::reply::with_status(error.to_string(), StatusCode::BAD_REQUEST))
    } else {
        event!(Level::WARN, "route not found");
        Ok(warp::reply::with_status(
            "route not found\n".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
