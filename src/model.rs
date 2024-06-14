use crate::types::{
    question::*,
    answer::*
};
use sqlx::{
    Error as sqlxError,
    postgres::{
        PgPool, PgPoolOptions, PgRow
    },
    Row
};

/// The database of questions and answers.
#[derive(Debug, Clone)]
pub struct Database {
    pub connection: PgPool
}

/// Potential errors while performing database operations.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Duplicate question or answer id
    #[error("Id already exists: {0}")]
    DuplicateId(String),

    /// Resource (usually a question) not found
    #[error("Resource not found")]
    NotFound,

    /// Generally unprocessable data/id
    #[error("Could not process data {0}")]
    UnprocessableData(String),

    /// For updating data, ids don't match
    #[error("Given ids don't match when they should: {0}, {1}")]
    MismatchedIds(String, String),

    /// Database couldn't be queried
    #[error("Database query error: {0}")]
    QueryError(String)
}
impl From<sqlxError> for DatabaseError {
    fn from(e: sqlxError) -> Self {
        DatabaseError::QueryError(e.to_string())
    }
}


impl Database {
    pub async fn new() -> Self {
        use std::env::var;

        let pw_file = var("PG_PASSWORDFILE")
            .expect("Error getting pw file env var");
        let pw = std::fs::read_to_string(pw_file)
            .expect("Couldn't read in pw");
        let db_url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER").expect("Couldn't get PG_USER var"),
            pw.trim(),
            var("PG_HOST").expect("Couldn't get PG_HOST var"),
            var("PG_DBNAME").expect("Couldn't get db name var")
        );

        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url).await
        {
            Ok(p) => p,
            Err(e) => panic!("Couldn't establish DB connection to {}! {}", db_url, e)
        };

        Database { connection: pool }
    }

    fn to_question(&self, r: &PgRow) -> Question {
        Question::new(
            r.get("id"), 
            r.get("title"), 
            r.get("content"), 
            r.get("tags")
        )
    }

    pub async fn get_questions(&self, limit: Option<i32>, offset: i32)
        -> Result<Vec<Question>, DatabaseError>
    {
        let qs = sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| self.to_question(&row))
            .fetch_all(&self.connection)
            .await?;

        Ok(qs)
    }

    pub async fn get_question_by_id(&self, qid: String) -> Result<Question, DatabaseError> {
        let row = sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(qid)
            .fetch_one(&self.connection)
            .await?;

        Ok(self.to_question(&row))
    }

    pub async fn add_question(&mut self, q: Question) -> Result<(), DatabaseError> {
        match sqlx::query("
                INSERT INTO questions (id, title, content, tags)
                VALUES ($1, $2, $3, $4)
            ")
            .bind(q.id.clone())
            .bind(q.title())
            .bind(q.content())
            .bind(q.tags())
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::QueryError(e.to_string()))
        }
    }

    pub async fn update_question(&mut self, qid: &str, new_q: QuestionUpdate)
        -> Result<(), DatabaseError>
    {
        match sqlx::query("
                UPDATE questions
                SET title = $1, content = $2, tags = $3
                WHERE id = $4
            ")
            .bind(new_q.title)
            .bind(new_q.content)
            .bind(new_q.tags)
            .bind(qid)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::QueryError(e.to_string()))
        }
    }

    pub async fn delete_question(&mut self, qid: &str) -> Result<(), DatabaseError> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(qid)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::QueryError(e.to_string()))
        }
    }

    pub async fn add_answer(&mut self, a: Answer) -> Result<(), DatabaseError> {
        match sqlx::query("
                INSERT INTO answers (id, content, orig_q)
                VALUES ($1, $2, $3)
            ")
            .bind(a.id.clone())
            .bind(a.content())
            .bind(a.question_id.clone())
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::QueryError(e.to_string()))
        }
    }

}