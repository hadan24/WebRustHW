mod question;
use question::*;

mod model;
use model::*;

use std::str::FromStr;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router
};

async fn question() -> Response {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:?}", question);

    (StatusCode::OK, Json(question)).into_response()
}

async fn handler_404() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found :(").into_response()
}

#[tokio::main]
async fn main() {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async { "Hallo :D 🦀" }))
        .route("/question", get(question))
        .fallback(handler_404);

    axum::serve(tcp_listener, app).await.unwrap();
}
