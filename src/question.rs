use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct QuestionId(String);
impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new
                (ErrorKind::InvalidInput, "No id provided")
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
impl Question {
    pub fn new (
        id: QuestionId, title: String,
        content: String, tags: Option<Vec<String>>
    ) -> Self {
        Question {id, title, content, tags}
    }
}