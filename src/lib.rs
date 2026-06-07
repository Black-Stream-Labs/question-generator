use poem_openapi::Object;

pub mod generator;

// TODO - decouple this from poem_openapi
// This will involve making wrappers in the API itself around the structs in
// this library, which might be doable with macros I've not discovered yet
#[derive(Object)]
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
