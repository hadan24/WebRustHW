mod question;
mod model;
mod routes;

use std::sync::Arc;
use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let db: model::Database = model::Database::new();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/questions", get(routes::questions))
        .route("/get_question", get(routes::get_question))
        .fallback(routes::handler_404)
        .with_state(Arc::new(db));

    axum::serve(tcp_listener, app).await.unwrap();
}
