use poem_openapi::Object;

pub mod generator;

#[derive(Object)]
pub struct Question {
    text: String,
    answers: Vec<String>,
    correct_answer: u32,
    explanation: Option<String>,
}
