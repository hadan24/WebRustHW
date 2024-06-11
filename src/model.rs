use crate::question::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type QuestionDB = HashMap<String, Question>;
#[derive(Clone, Serialize, Deserialize)]
pub struct Database {
    pub questions: QuestionDB
}

impl Database {
    pub fn new() -> Self {
        Database {questions: Self::init()}
    }
    fn init() -> QuestionDB {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Couldn't read questions.json :(")
    }

    fn add_question(mut self, q: Question) -> Self {
        self.questions.insert(q.id.clone(), q);
        self
    }
}