use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Question {
    pub id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}