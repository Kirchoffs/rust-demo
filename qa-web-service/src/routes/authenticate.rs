use std::{env, future};

use argon2::Config;
use chrono::Utc;
use rand::Rng;
use warp::Filter;

use crate::types::account::{Account, AccountId, Session};
use crate::store::Store;

pub async fn register(account: Account, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());
    let account = Account {
        id: None,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status(
            "account created",
            warp::http::StatusCode::CREATED,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(account: Account, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(account.email).await {
        Ok(stored_account) => match verify_password(&stored_account.password, account.password.as_bytes()) {
            Ok(veriried) => {
                if veriried {
                    Ok(warp::reply::json(&issue_token(stored_account.id.expect("id not found"))))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            },
            Err(err) => Err(warp::reject::custom(handle_errors::Error::ArgonLibraryError(err))),
        }
        Err(err) => Err(warp::reject::custom(err)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    let key = env::var("PASETO_KEY").unwrap();
    
    let current_date_time = Utc::now();
    let expiration_date_time = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&expiration_date_time)
        .set_not_before(&current_date_time)
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("failed to construct paseto token")
}

fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();

    let token = paseto::tokens::validate_local_token(
        &token, 
        None, 
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono
    ).map_err(|_| handle_errors::Error::CannotVerifyToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotVerifyToken)
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and_then(|token: String| {
            let session = match verify_token(token) {
                Ok(session) => session,
                Err(_) => return future::ready(Err(warp::reject::reject())),
            };

            future::ready(Ok(session))
        })
}
