mod question;
mod model;
mod routes;

use std::sync::Arc;
use tokio::{
    self,
    sync::RwLock
};
use axum::{
    routing::{get, post},
    Router
};

#[tokio::main]
async fn main() {
    let db = model::Database::new();
    let db = Arc::new(RwLock::new(db));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/questions", get(routes::questions))
        .route("/questions/:id", get(routes::get_question))
        .route("/add_question", post(routes::post_question))
        .fallback(routes::handler_404)
        .with_state(db);

    axum::serve(tcp_listener, app).await.unwrap();
}
