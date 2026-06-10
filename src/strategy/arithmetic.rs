//! Generate maths questions
//!
//! Provides an interface to generate maths questions, using the generic GeneratorParameters from
//! the crate as well as its own generator parameters struct.
use crate::{
    Question,
    GeneratorParameters,
    KeyStage,
    string_to_enum_vec
};
use rand::seq::{IndexedRandom, SliceRandom};
use std::str::FromStr;
use strum_macros::EnumString;

mod addition;

#[cfg(feature = "poem")]
use serde::Deserialize;

#[derive(EnumString)]
#[cfg_attr(feature = "poem", derive(Deserialize))]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

/// Interface into generating questions.
///
/// Honoured values from the curriculum will be area, stage, level, and difficulty.
/// Currently, area is used to define what operations to use, and difficulty will control the size
/// of numbers used in the problems. Stage and level will be different ways of controlling the
/// types of questions generated, e.g. simple sums or BIDMAS, division with or without remainder or
/// fractional parts.
pub fn generate(params: &GeneratorParameters) -> Vec<Question> {
    let mut questions : Vec<Question> = vec![];

    let ops = params.curriculum.area.clone().unwrap_or("Addition".to_string());
    let operations : Vec<ArithmeticOperation> = string_to_enum_vec(&ops).unwrap();

    for _ in 0 .. params.count {
        let operation = operations.choose(&mut rand::rng()).unwrap();
        let question = match operation {
            ArithmeticOperation::Addition => addition::generate_addition(&params),
            ArithmeticOperation::Subtraction => generate_subtraction(&params),
            ArithmeticOperation::Multiplication => generate_multiplication(&params),
            ArithmeticOperation::Division => generate_division(&params),
        };

        questions.push(question);
    }

    return questions;
}

// Generates `count` answers, including `correct_answer`.
//
// Answers will be taken from a range `spread` wide centred around `correct_answer` (sort of -
// depending on the parity of `spread`, it may be 1 bigger).
//
// If allow_negative is false, the range is adjusted so that the lowest value is 1, which means
// `correct_answer` will not be in the middle of the range if it is too close to zero.
fn generate_answers(correct_answer:i32, count:usize, spread: i32, allow_negative: bool) -> (Vec<String>, usize) {
    // Sometimes we compare count to spread, but it's really supposed to represent the size of the
    // vector, so make a copy for the peripheral logic
    let count_i32 = count as i32;

    assert!(spread >= count_i32, "Spread must be at least as big as count!");

    let range = if allow_negative {
        -i32::midpoint(spread, count_i32) ..= i32::midpoint(spread, count_i32)
    }
    else {
        // If the lowest value would be negative, work out what it would be and add it back on,
        // plus 1 so it's not zero
        let offset = if spread/2 > correct_answer { spread/2 - correct_answer + 1 } else { 0 };
        count_i32 + offset ..= spread + count_i32 + offset
    };

    let mut wrong_answers : Vec<i32> = range.collect();
    wrong_answers.retain(|x| *x != correct_answer);
    wrong_answers.shuffle(&mut rand::rng());

    let mut answers = wrong_answers[0..count].to_vec();

    let correct_answer_idx : usize = rand::random_range(0..answers.len());
    answers.insert(correct_answer_idx, correct_answer);

    (answers.iter().map(std::string::ToString::to_string).collect(), correct_answer_idx)
}

// Stage/level:
//   Key Stage 1 - positive integers and zero
//      Difficulty 1 - single-digit numbers
//                 2 - maybe double-digit numbers, single-digit answers
//                 3 - double-digit numbers, double-digit answers
//                 4 - 3-digit - 1-digit
//                 5 - 3-digit - 1/2/3-digit
//   Key Stage 2 -
//   Key Stage 3 -
fn generate_subtraction(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    let allow_negative = false; // Something based on params.curriculum

    let correct_answer = if allow_negative {
        num_1 - num_2
    }
    else {
        let bignum = cmp::max(num_1, num_2);
        let smlnum = cmp::min(num_1, num_2);
        bignum - smlnum
    };

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20, allow_negative);

    Question {
        text: "`num_1` - `num_2` = ?".to_string(),
        answers,
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

// Stage/level: (FIXME)
//   Key Stage 1 - 2, 5, 10 times tables
//      Difficulty 1 - single-digit answers, or 10×<10
//                 2 - double-digit answers, or up to 10×12
//                 3 - up to 15×N
//                 4 - sometimes 3 numbers, but answers up to 120
//                 5 - sometimes 3 numbers, but answers up to 150
//   Key Stage 2 -
//   Key Stage 3 -
fn generate_multiplication(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    let correct_answer = num_1 * num_2;

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20, false);

    Question {
        text: "`num_1` × `num_2` = ?".to_string(),
        answers,
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

// Stage/level: (FIXME)
//   Key Stage 1 - 2, 5, 10 times tables
//      Difficulty 1 - single-digit answers, or 10×<10
//                 2 - double-digit answers, or up to 10×12
//                 3 - up to 15×N
//   Key Stage 2 -
//   Key Stage 3 -
fn generate_division(params: &GeneratorParameters) -> Question {
    // TODO - here we inspect the curriculum (not yet implemented) to decide
    // which types of division we're going to use
    generate_integer_division(params)
}

fn generate_integer_division(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    // This ensures integer division because we ask what is numerator / num_1
    let numerator = num_1 * num_2;

    let correct_answer = num_2;

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20, false);

    Question {
        text: "`numerator` ÷ `num_1` = ?".to_string(),
        answers,
        correct_answer: correct_answer_idx,
        explanation: Some("`numerator` ÷ `num_1` = `num_2` because `num_1` × `num_2` = `numerator`!".to_string())
    }
}

fn generate_integer_division_with_remainder(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);
    // The remainder will be less than num1 because num1 is the denominator
    let remainder = rand::random_range(1..num_1);

    // This ensures integer division because we ask what is numerator / num_1
    let numerator = (num_1 * num_2) + remainder;

    let correct_answer = num_2;

    // Have to do this bespoke
    let mut answers : Vec<String> = vec![];

    // FIXME - harder to exclude duplicates in here
    for _ in 1 .. params.count {
        let random_answer = correct_answer + (0 - rand::random_range(1..10)/2);
        let random_answer_remainder = rand::random_range(1..num_1);

        answers.push("`random_answer` remainder `random_answer_remainder`".to_string());
    }

    let correct_answer_idx : usize = rand::random_range(0..answers.len());
    answers.insert(correct_answer_idx, "`correct_answer` remainder `remainder`".to_string());

    Question {
        text: "`numerator` ÷ `num_1` = ?".to_string(),
        answers,
        correct_answer: correct_answer_idx,
        explanation: None //Some(format!("{} ÷ {} = {} because {} × {} = {}!", numerator, num_1, num_2, num_1, num_2, numerator))
    }
}
