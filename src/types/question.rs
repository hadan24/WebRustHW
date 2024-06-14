use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Question {
    pub id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: String, title: String, content: String, tags: Option<Vec<String>>)
        -> Self
    {
        Question {id, title, content, tags}
    }

    pub fn title(&self) -> &String { &self.title }
    pub fn content(&self) -> &String { &self.content }
    pub fn tags(&self) -> &Option<Vec<String>> { &self.tags }
}

#[derive(Debug, Deserialize, Hash, Clone)]
pub struct QuestionUpdate {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}