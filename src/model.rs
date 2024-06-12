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
    DuplicateId(String)
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

    pub fn get_question_by_id(&self, qid: String) -> Option<&Question> {
        self.questions.get(&qid)
    }

    pub fn add_question(&mut self, q: Question) -> Result<(), DatabaseError> {
        let qid = q.id.clone();

        if self.questions.get(&qid).is_some() {
            return Err(DatabaseError::DuplicateId(q.id));
        }

        self.questions.insert(qid, q);
        Ok(())
    }

}