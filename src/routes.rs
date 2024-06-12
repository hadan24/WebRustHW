use crate::question::*;
use crate::model::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    start: Option<usize>,
    end: Option<usize>
}
pub async fn questions (
    State(db): State<Arc<RwLock<Database>>>,
    Query(args): Query<PaginationParams>
) -> Response {
    let sorted_data = db.read().await.get_sorted_data();

    match args {
        PaginationParams { start: None, end: Some(_) } |
        PaginationParams { start: Some(_), end: None } =>
            (StatusCode::BAD_REQUEST, Json("400 Bad Request >:(")).into_response(),

        PaginationParams { start: Some(x), end: Some(y) } => {
            let y = y + 1;  // make the end index match the user's expectations
            if x > y || x > sorted_data.len() {
                (StatusCode::BAD_REQUEST, Json("400 Bad Request >:(")).into_response()
            } else if y+1 > sorted_data.len() {
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

pub async fn get_question (
    State(db): State<Arc<RwLock<Database>>>,
    Path(qid): Path<String>
) -> Response {
    match db.read().await.get_question_by_id(&qid) {
        Some(q) => (StatusCode::OK, Json(q)).into_response(),
        None => (StatusCode::NOT_FOUND, Json("404 Not Found :(")).into_response()
    }
}

pub async fn post_question (
    State(db): State<Arc<RwLock<Database>>>,
    Json(q): Json<Question>
) -> Response {
    match db.write().await.add_question(q) {
        Ok(_) => (StatusCode::CREATED, "Question posted!").into_response(),
        Err(e) =>
            (StatusCode::BAD_REQUEST, format!("Question already exists: {:?}", e))
            .into_response()
    }
}

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
        Err(DatabaseError::UnprocessableId(e_id)) =>
            (StatusCode::UNPROCESSABLE_ENTITY, format!("Couldn't process id: {:?}", e_id))
            .into_response(),
        Err(e) =>
            (StatusCode::BAD_REQUEST, "Otherwise faulty request")
            .into_response()
    }
}

pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}