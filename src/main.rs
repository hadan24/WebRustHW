mod question;
use question::*;
use std::str::FromStr;
use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );
    println!("{:?}", question);

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let app = Router::new()
        .route("/", get(|| async {"Hallo :D 🦀"}) );

    axum::serve(tcp_listener, app).await.unwrap();
}
