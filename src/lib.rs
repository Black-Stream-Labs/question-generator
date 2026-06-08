#[cfg(feature = "poem")]
use poem_openapi::Object;
#[cfg(feature = "poem")]
use serde::Deserialize;

pub mod generator;

#[cfg_attr(feature = "poem", derive(Object))]
pub struct Question {
    text: String,
    answers: Vec<String>,
    correct_answer: usize,
    explanation: Option<String>,
}

pub struct GeneratorParameters {
    pub count: usize,
    pub answer_count: usize
}
