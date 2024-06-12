use crate::question::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type QuestionDB = HashMap<String, Question>;
#[derive(Clone, Serialize, Deserialize)]
pub struct Database {
    pub questions: QuestionDB
}

#[derive(Debug)]
pub enum DatabaseError {
    DuplicateId(String),
    NotFound,
    UnprocessableId(String)
}
impl Database {
    pub fn new() -> Self {
        Database {questions: Self::init()}
    }
    fn init() -> QuestionDB {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Couldn't read questions.json :(")
    }

    pub fn get_sorted_data(&self) -> Vec<Question> {
        let mut data: Vec<Question> = self.questions.values().cloned().collect();
        data.sort_by(|a, b| a.id.cmp(&b.id));
        data
    }

    pub fn get_question_by_id(&self, qid: &str) -> Option<&Question> {
        self.questions.get(&qid.to_string())
    }

    pub fn add_question(&mut self, q: Question) -> Result<(), DatabaseError> {
        let qid = q.id.clone();

        match self.questions.get(&qid) {
            Some(_) => Err(DatabaseError::DuplicateId(q.id)),
            None => {
                self.questions.insert(qid, q);
                Ok(())
            }
        }
    }

    pub fn update_question(&mut self, qid: &str, new_q: Question)
        -> Result<(), DatabaseError>
    {
        if qid.is_empty() {
            return Err(DatabaseError::UnprocessableId(qid.to_string()));
        }

        match self.questions.get_mut(&qid.to_string()) {
            Some(q) => {
                *q = new_q;
                Ok(())
            },
            None => Err(DatabaseError::NotFound)
        }
    }

}