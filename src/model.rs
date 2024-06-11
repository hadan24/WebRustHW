use crate::question::*;
use std::{
    collections::HashMap,
    str::FromStr
};

struct Database {
    questions: HashMap<QuestionId, Question>
}

impl Database {
    fn new() -> Self {
        Database {questions: HashMap::new()}
    }

    fn init(self) -> Self {
        let q = Question::new(
            QuestionId::from_str("1").expect("No id provided"),
            "First Question".to_string(),
            "Content of question".to_string(),
            Some(vec!["faq".to_string()]),
        );
        self.add_question(q)
    }

    fn add_question(mut self, q: Question) -> Self {
        self.questions.insert(q.id.clone(), q);
        self
    }
}