use crate::question::*;
use crate::model::*;
use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

pub async fn questions (
    State(db): State<Arc<Database>>
) -> Response {
    let data: Vec<Question> = db.questions.values().cloned().collect();

    (StatusCode::OK, Json(&*data)).into_response()
}

pub async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}