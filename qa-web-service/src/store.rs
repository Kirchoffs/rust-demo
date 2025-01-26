use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use tracing::{event, Level};

use crate::types::{
    answer::Answer,
    question::{Question, QuestionId},
};

use handle_errors::Error;

use crate::types::{
    account::{Account, AccountId},
    answer::NewAnswer,
    question::NewQuestion
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        Ok(Store {
            connection: db_pool,
        })
    }
    
    pub async fn get_questions(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(err) => {
                    event!(Level::ERROR, "failed to get questions: {:?}", err);
                    Err(Error::DatabaseQueryError(err))
                }
            }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags, account_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags;"
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .bind(account_id)
        .map(|row: PgRow| Question {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(err) => {
                event!(Level::ERROR, "failed to add question: {:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn update_question(
        &self,
        question_id: QuestionId,
        question: Question,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 and account_id = $5 RETURNING id, title, content, tags;"
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .bind(account_id)
        .map(|row: PgRow| Question {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(question) => Ok(question),
            Err(err) => {
                event!(Level::ERROR, "failed to update question: {:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn delete_question(&self, question_id: QuestionId) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1;")
            .bind(question_id)
            .execute(&self.connection)
            .await {
                Ok(_) => Ok(true),
                Err(err) => {
                    event!(Level::ERROR, "failed to delete question: {:?}", err);
                    Err(Error::DatabaseQueryError(err))
                }
            }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer, account_id: AccountId) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INTO answers (content, question_id, account_id) VALUES ($1, $2, $3) RETURNING id, content, question_id;"
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id)
        .bind(account_id)
        .map(|row: PgRow| Answer {
            id: row.get("id"),
            content: row.get("content"),
            question_id: row.get("question_id"),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(answer) => Ok(answer),
            Err(err) => {
                event!(Level::ERROR, "failed to add answer: {:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn add_account(&self, account: Account) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO accounts (email, password) VALUES ($1, $2);"
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await {
            Ok(_) => Ok(true),
            Err(err) => {
                event!(
                    Level::ERROR,
                    code = err.as_database_error().unwrap().code().unwrap().parse::<i32>().unwrap(),
                    db_message = err.as_database_error().unwrap().message(),
                    constraint = err.as_database_error().unwrap().constraint().unwrap(),
                );
                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn get_account(&self, email: String) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM accounts WHERE email = $1;")
            .bind(email)
            .map(|row: PgRow| Account {
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await {
                Ok(account) => Ok(account),
                Err(err) => {
                    event!(Level::ERROR, "failed to get account: {:?}", err);
                    Err(Error::DatabaseQueryError(err))
                }
            }
    }

    pub async fn is_question_owner(&self, question_id: QuestionId, account_id: AccountId) -> Result<bool, Error> {
        match sqlx::query(
            "SELECT * FROM questions WHERE id = $1 AND account_id = $2;"
        )
        .bind(question_id)
        .bind(account_id)
        .fetch_optional(&self.connection)
        .await {
            Ok(question) => Ok(question.is_some()),
            Err(err) => {
                event!(Level::ERROR, "failed to check if question owner: {:?}", err);
                Err(Error::DatabaseQueryError(err))
            }
        }
    }
}
