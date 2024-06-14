use crate::types::{
    question::*,
    answer::*
};
use crate::model::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::Deserialize;
use nanoid::nanoid;

/// Pagination query parameters, includes the `start`
/// and `end` indices of sorted questions to display
#[derive(Deserialize)]
pub struct PaginationParams {
    /// Index of last item in page
    pub limit: Option<i32>,
    /// Index of first item in page
    pub offset: i32
}
/// Displays all questions or a subset of questions sorted by id.
/// If no pagination query was provided, display everything.
/// # Example queries:
/// `/questions?start=0&end=10`
/// `/questions?start=10    // Displays ALL questions at and after index 10`
pub async fn questions (
    State(db): State<Arc<RwLock<Database>>>,
    Query(args): Query<PaginationParams>
) -> Response {
    match db.read().await.get_questions(args.limit, args.offset).await {
        Ok(qs) => (StatusCode::OK, Json(&qs)).into_response(),
        Err(e) =>
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {:?}", e))
            .into_response()
    }
}

/// Displays a question based on the id given in the path.
/// # Example request:
/// `/questions/help`
pub async fn get_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>
) -> Response {
    match db.read().await.get_question_by_id(qid).await {
        Ok(q) => (StatusCode::OK, Json(q)).into_response(),
        Err(_e) => (StatusCode::NOT_FOUND, Json("404 Not Found :(")).into_response()
    }
}

/// Posts a new question in JSON form to the database.
pub async fn post_question (
    State(db): State<Arc<RwLock<Database>>>,
    Json(q): Json<Question>
) -> Response {
    match db.write().await.add_question(q).await {
        Ok(_) => (StatusCode::CREATED, "Question posted!").into_response(),
        Err(e) =>
            (StatusCode::CONFLICT, format!("Question id already exists: {:?}", e))
            .into_response()
    }
}

/// Updates an existing question. The question id should be included in
/// the path, and the updated content should come in JSON format.
pub async fn update_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>,
    Json(q): Json<QuestionUpdate>
) -> Response {
    match db.write().await.update_question(&qid, q).await {
        Ok(_) => (StatusCode::OK, "Question updated!").into_response(),
        Err(e) =>
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Issue updating question: {:?}", e))
            .into_response()
    }
}

/// Deletes a question based on the id given in the path.
pub async fn delete_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>
) -> Response {
    match db.write().await.delete_question(&qid).await {
        Ok(_) => (StatusCode::OK, "Question deleted.").into_response(),
        Err(e) =>
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Issue deleting question: {:?}", e))
            .into_response()
    }
}

/// Posts an answer to a particular question.
/// The question's id shold be included in the path, and
/// the answer's content should be included in a query.
pub async fn post_answer (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>,
    Query(answer): Query<String>
) -> Response {
    let a = Answer::new(nanoid!(), answer, qid);

    match db.write().await.add_answer(a).await {
        Ok(_) => (StatusCode::OK, "Answer posted!").into_response(),
        Err(e) =>
            (StatusCode::CONFLICT, format!("Issue posting answer: {:?}", e))
            .into_response()
    }
}

// Handles any requests that do not match the others.
pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}