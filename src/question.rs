use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Question {
    pub id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
impl Question {
    pub fn new (
        id: String, title: String,
        content: String, tags: Option<Vec<String>>
    ) -> Self {
        Question {id, title, content, tags}
    }
}