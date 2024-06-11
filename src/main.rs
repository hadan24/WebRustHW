mod question;
use question::*;
mod model;
use model::*;

use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router
};

async fn question(
    State(db): State<Arc<Database>>
) -> Response {
    let data: Vec<Question> = db.questions.values().cloned().collect();

    (StatusCode::OK, Json(&*data)).into_response()
}

async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}

#[tokio::main]
async fn main() {
    let db: Database = model::Database::new();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/question", get(question))
        .fallback(handler_404)
        .with_state(Arc::new(db));

    axum::serve(tcp_listener, app).await.unwrap();
}
