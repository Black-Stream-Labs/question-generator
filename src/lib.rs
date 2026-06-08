#[cfg(feature = "poem")]
use poem_openapi::Object;
#[cfg(feature = "poem")]
use serde::Deserialize;

pub mod strategy;

#[cfg_attr(feature = "poem", derive(Object))]
pub struct Question {
    pub text: String,
    pub answers: Vec<String>,
    pub correct_answer: usize,
    pub explanation: Option<String>,
}

pub struct Curriculum {
    pub subject: String,
    pub area: Option<String>,
    pub stage: Option<String>,
    pub interest_level: Option<String>,
    pub difficulty: usize,
}

pub struct GeneratorParameters {
    pub count: usize,
    pub answer_count: usize,
    pub curriculum: Curriculum,
}
