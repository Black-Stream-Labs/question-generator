use crate::{Question, GeneratorParameters};
use rand::seq::IndexedRandom;
use std::cmp;

pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    IntegerDivision,
    IntegerDivisionWithRemainder,
    //DivisionWithFraction,
//    DivisionWithDecimal
}

pub struct MathsGeneratorParameters {
    pub operations: Vec<ArithmeticOperation>
}


pub fn generate(params: GeneratorParameters, maths_params: MathsGeneratorParameters) -> Vec<Question> {
    let mut questions : Vec<Question> = vec![];

    for _ in 0 .. params.count {
        let operation = maths_params.operations.choose(&mut rand::rng()).unwrap();
        let question = match operation {
            ArithmeticOperation::Addition => generate_addition(&params),
            ArithmeticOperation::Subtraction => generate_subtraction(&params),
            ArithmeticOperation::Multiplication => generate_multiplication(&params),
            ArithmeticOperation::IntegerDivision => generate_integer_division(&params),
            ArithmeticOperation::IntegerDivisionWithRemainder => generate_integer_division_with_remainder(&params),
        };

        questions.push(question);
    }

    return questions;
}

// FIXME - this isn't entirely so useful because the sensible range of answers
// depends on the question type, e.g. additions can't come out negative!
fn generate_answers(correct_answer:i32, count:usize, spread: i32) -> (Vec<String>, usize) {
    let mut answers : Vec<String> = vec![];

    for _ in 1 .. count {
        let random_answer = correct_answer + (0 - rand::random_range(1..spread)/2);
        answers.push(random_answer.to_string());
    }

    let correct_answer_idx : usize = rand::random_range(0..answers.len());
    answers.insert(correct_answer_idx, correct_answer.to_string());

    return (answers, correct_answer_idx);
}

fn generate_addition(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    let correct_answer = num_1 + num_2;

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20);

    Question {
        text: format!("{} + {} = ?", num_1, num_2),
        answers: answers,
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

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

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20);

    Question {
        text: format!("{} + {} = ?", num_1, num_2),
        answers: answers,
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

fn generate_multiplication(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    let correct_answer = num_1 * num_2;

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20);

    Question {
        text: format!("{} × {} = ?", num_1, num_2),
        answers: answers,
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

fn generate_integer_division(params: &GeneratorParameters) -> Question {
    let num_1 = rand::random_range(1..10);
    let num_2 = rand::random_range(1..10);

    // This ensures integer division because we ask what is numerator / num_1
    let numerator = num_1 * num_2;

    let correct_answer = num_2;

    let (answers, correct_answer_idx) = generate_answers(correct_answer, params.answer_count, 20);

    Question {
        text: format!("{} ÷ {} = ?", numerator, num_1),
        answers: answers,
        correct_answer: correct_answer_idx,
        explanation: Some(format!("{} ÷ {} = {} because {} × {} = {}!", numerator, num_1, num_2, num_1, num_2, numerator))
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

        answers.push(format!("{} remainder {}", random_answer, random_answer_remainder));
    }

    let correct_answer_idx : usize = rand::random_range(0..answers.len());
    answers.insert(correct_answer_idx, format!("{} remainder {}", correct_answer, remainder));

    Question {
        text: format!("{} ÷ {} = ?", numerator, num_1),
        answers: answers,
        correct_answer: correct_answer_idx,
        explanation: None //Some(format!("{} ÷ {} = {} because {} × {} = {}!", numerator, num_1, num_2, num_1, num_2, numerator))
    }
}
