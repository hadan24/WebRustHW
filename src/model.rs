use crate::question::*;
use crate::answer::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type QuestionDB = HashMap<String, Question>;
pub type AnswerDB = HashMap<String, Answer>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Database {
    pub questions: QuestionDB,
    pub answers: AnswerDB
}

#[derive(Debug)]
pub enum DatabaseError {
    DuplicateId(String),
    NotFound,
    UnprocessableId(String)
}
impl Database {
    pub fn new() -> Self {
        let file = include_str!("../questions.json");

        Database {
            questions: serde_json::from_str(file)
                .expect("Couldn't read questions.json :("),
            answers: HashMap::new()
        }
    }

    pub fn get_sorted_data(&self) -> Vec<Question> {
        let mut data: Vec<Question> = self.questions.values().cloned().collect();
        data.sort_by(|a, b| a.id.cmp(&b.id));
        data
    }

    pub fn get_question_by_id(&self, qid: &str) -> Option<&Question> {
        self.questions.get(qid)
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

        match self.questions.get_mut(qid) {
            Some(q) => {
                *q = new_q;
                Ok(())
            },
            None => Err(DatabaseError::NotFound)
        }
    }

    pub fn delete_question(&mut self, qid: &str) -> Result<(), DatabaseError> {
        if qid.is_empty() {
            return Err(DatabaseError::UnprocessableId(qid.to_string()));
        }

        match self.questions.remove(qid) {
            Some(_) => Ok(()),
            None => Err(DatabaseError::NotFound)
        }
    }

    pub fn add_answer(&mut self, a: Answer) -> Result<(), DatabaseError> {
        let a_id = a.id.clone();

        match self.answers.get(&a_id) {
            Some(_) => Err(DatabaseError::DuplicateId(a_id)),
            None => {
                if self.questions.contains_key(&a.question_id) {
                    self.answers.insert(a_id, a);
                    Ok(())
                } else {
                    Err(DatabaseError::NotFound)
                }
            }
        }
    }

}