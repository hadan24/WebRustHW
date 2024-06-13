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
    start: Option<usize>,
    end: Option<usize>
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
    let sorted_data = db.read().await.get_sorted_data();

    match args {
        PaginationParams { start: None, end: Some(_) } => {
            (StatusCode::BAD_REQUEST, Json("400 Bad Request. No starting index"))
                .into_response()
        },

        PaginationParams { start: Some(x), end: None } => {
            if x > sorted_data.len() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(format!("400 Bad Request. No entries past index {:?}", x))
                ).into_response();
            }
            (StatusCode::OK, Json(&sorted_data[x..])).into_response()
        },

        PaginationParams { start: Some(x), end: Some(y) } => {
            let y = y + 1;  // make the end index match the user's expectations
            if x > y || x > sorted_data.len() {
                (StatusCode::BAD_REQUEST, Json("400 Bad Request. Check indices."))
                    .into_response()
            } else if y > sorted_data.len() {
                (StatusCode::OK, Json(&sorted_data[x..])).into_response()
            } else {
                (StatusCode::OK, Json(&sorted_data[x..y])).into_response()
            }
        },

        PaginationParams { start: None, end: None } => {
            (StatusCode::OK, Json(sorted_data)).into_response()
        }
    }
}

/// Displays a question based on the id given in the path.
/// # Example request:
/// `/questions/help`
pub async fn get_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>
) -> Response {
    match db.read().await.get_question_by_id(&qid) {
        Some(q) => (StatusCode::OK, Json(q)).into_response(),
        None => (StatusCode::NOT_FOUND, Json("404 Not Found :(")).into_response()
    }
}

/// Posts a new question in JSON form to the database.
pub async fn post_question (
    State(db): State<Arc<RwLock<Database>>>,
    Json(q): Json<Question>
) -> Response {
    match db.write().await.add_question(q) {
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
    Json(q): Json<Question>
) -> Response {
    match db.write().await.update_question(&qid, q) {
        Ok(_) => (StatusCode::OK, "Question updated!").into_response(),
        Err(DatabaseError::NotFound) =>
            (StatusCode::NOT_FOUND, "Couldn't find question")
            .into_response(),
        Err(DatabaseError::UnprocessableData(e_id)) =>
            (StatusCode::UNPROCESSABLE_ENTITY, format!("Couldn't process id: {:?}", e_id))
            .into_response(),
        Err(DatabaseError::MismatchedIds(e1, e2)) =>
            (StatusCode::BAD_REQUEST, format!("Provided ids don't match: {:?}, {:?}", e1, e2))
            .into_response(),
        Err(_e) =>
            (StatusCode::INTERNAL_SERVER_ERROR, "Otherwise faulty request").into_response()
    }
}

/// Deletes a question based on the id given in the path.
pub async fn delete_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>
) -> Response {
    match db.write().await.delete_question(&qid) {
        Ok(_) => (StatusCode::OK, "Question deleted.").into_response(),
        Err(DatabaseError::NotFound) =>
            (StatusCode::NOT_FOUND, format!("Couldn't find question with id: {:?}", qid))
            .into_response(),
        Err(DatabaseError::UnprocessableData(e_id)) =>
            (StatusCode::UNPROCESSABLE_ENTITY, format!("Couldn't process id: {:?}", e_id))
            .into_response(),
        Err(_) =>
            (StatusCode::INTERNAL_SERVER_ERROR, "This should not be reached.")
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

    match db.write().await.add_answer(a) {
        Ok(_) => (StatusCode::OK, "Answer posted!").into_response(),
        Err(e) =>
            (StatusCode::CONFLICT, format!("Duplicate answer id: {:?}", e))
            .into_response()
    }
}

// Handles any requests that do not match the others.
pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}