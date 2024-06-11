use crate::question::*;
use crate::model::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Query, State},
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
    let mut data: Vec<Question> = db.read().await.questions
        .values().cloned().collect();
    data.sort_by(|a, b| a.id.cmp(&b.id));

    match args {
        PaginationParams { start: None, end: Some(_) } |
        PaginationParams { start: Some(_), end: None } =>
            (StatusCode::BAD_REQUEST, "400 Bad Request >:(").into_response(),

        PaginationParams { start: Some(x), end: Some(y) } => {
            let y = y + 1;  // make the end index match the user's expectations
            if x > y || x > data.len() {
                (StatusCode::BAD_REQUEST, "400 Bad Request >:(").into_response()
            } else if y+1 > data.len() {
                (StatusCode::OK, Json(&data[x..])).into_response()
            } else {
                (StatusCode::OK, Json(&data[x..y])).into_response()
            }
        },
        PaginationParams { start: None, end: None } => {
            (StatusCode::OK, Json(data)).into_response()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId { id: String }
pub async fn get_question(
    State(db): State<Arc<RwLock<Database>>>,
    Query(qid): Query<QuestionId>
) -> Response {
    match db.read().await.questions.get(&qid.id) {
        Some(q) => (StatusCode::OK, Json(q)).into_response(),
        None => (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
    }
}

pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}