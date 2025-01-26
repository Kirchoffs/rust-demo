use warp::http::StatusCode;

use crate::{profanity::check_profanity, types::{account::Session, answer::NewAnswer}};
use crate::store::Store;

pub async fn add_answer(
    new_answer: NewAnswer,
    store: Store,
    session: Session
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    let content = match check_profanity(new_answer.content.clone()).await {
        Ok(content) => content,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer, account_id).await {
        Ok(_) => {
            Ok(warp::reply::with_status("answer added", StatusCode::OK))
        },
        Err(err) => Err(warp::reject::custom(err)),
    }
}
