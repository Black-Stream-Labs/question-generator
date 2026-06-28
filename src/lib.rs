#![warn(clippy::pedantic)]
#[cfg(feature = "poem")]
use poem_openapi::Object;
#[cfg(feature = "poem")]
use serde::Deserialize;

use std::str::FromStr;
use strum_macros::EnumString;

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
    pub count: u16,
    pub answer_count: u16,
    pub curriculum: Curriculum,
}

// Everything is passed by string between the user of the lib and the
// generator strategies, to allow the strategies to extend any of the enums
// at runtime. But we create some base enums anyway for things that are going
// to be common across all curricula, just to avoid repetition.
#[derive(EnumString)]
pub enum KeyStage {
    Foundation, KeyStage1, KeyStage2, KeyStage3, KeyStage4
}

fn string_to_enum_vec<T: FromStr>(string: &String) -> Result<Vec<T>, <T as FromStr>::Err> {
    string
        .split(',')
        .collect::<Vec<_>>().iter()
        .map(|o| T::from_str(o))
        .collect()
}
