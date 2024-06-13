use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: String,
    content: String,
    pub question_id: String
}

impl Answer {
    pub fn new(id: String, content: String, qid: String) -> Self {
        Answer { id, content, question_id: qid }
    }
}