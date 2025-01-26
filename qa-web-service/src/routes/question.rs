use std::collections::HashMap;

use tracing::{event, info, Level};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    profanity::check_profanity, 
    types::{
        account::Session, 
        pagination::{extract_pagination, Pagination}, 
        question::{NewQuestion, Question, QuestionId},
    },
    store::Store,
};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store
) -> Result<impl Reply, Rejection> {
    event!(target: "qa-web-service", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    info!(pagination = false);
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await {
            Ok(res) => res,
            Err(err) => {
                return Err(warp::reject::custom(err));
            }
        };
    
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    new_question: NewQuestion,
    store: Store,
    session: Session
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;

    let title = tokio::spawn(check_profanity(new_question.title));
    let content = tokio::spawn(check_profanity(new_question.content));

    let (title, content) = (title.await.unwrap(), content.await.unwrap());

    if title.is_err() {
        return Err(warp::reject::custom(title.err().unwrap()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.err().unwrap()));
    }

    let question = NewQuestion {
        title: title.unwrap(),
        content: content.unwrap(),
        tags: new_question.tags,
    };

    match store.add_question(question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e))
    }
}

pub async fn update_question(
    question_id: QuestionId,
    question: Question,
    store: Store,
    session: Session
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(question_id, account_id).await.is_err() {
        return Err(warp::reject::custom(handle_errors::Error::Unauthorized));
    }

    let title = check_profanity(question.title);
    let content = check_profanity(question.content);

    let (title, content) = tokio::join!(title, content);

    if title.is_err() {
        return Err(warp::reject::custom(title.err().unwrap()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.err().unwrap()));
    }

    let question = Question {
        id: question_id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

    let res = match store.update_question(question_id, question, account_id).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    question_id: QuestionId,
    store: Store,
    session: Session
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(question_id, account_id).await.is_err() {
        return Err(warp::reject::custom(handle_errors::Error::Unauthorized));
    }

    if let Err(err) = store.delete_question(question_id).await {
        return Err(warp::reject::custom(err));
    }

    Ok(warp::reply::with_status(
        format!("question with id {} deleted", question_id),
        StatusCode::OK,
    ))
}
