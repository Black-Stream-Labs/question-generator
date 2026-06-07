use crate::Question;

pub fn generate() -> Vec<Question> {
    vec![Question {
        text: "some_text".to_string(),
        answers: vec![ "A".to_string(), "B".to_string(), "C".to_string() ],
        correct_answer: 0,
        explanation: None
    }]
}
