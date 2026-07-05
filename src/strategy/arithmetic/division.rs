// Stage/level: (FIXME)
//   Key Stage 1 - 2, 5, 10 times tables
//      Difficulty 1 - numerator 1..9
//                 2 - numerator 1..15
//                 3 - numerator 1..20
//   Key Stage 2 -
//   Key Stage 3 -
use rand::seq::{IndexedRandom, SliceRandom};
use std::ops::Range;
use std::cmp::{PartialOrd, PartialEq};
use num_traits::{Num, NumAssignOps};

use crate::{
    Question,
    GeneratorParameters,
    KeyStage,
    string_to_enum_vec
};
use super::generate_wrong_answers_int;

pub fn generate_division(params: &GeneratorParameters) -> Question {
    let keystages : Vec<KeyStage> = string_to_enum_vec(& params.curriculum.stage.clone().unwrap()).unwrap();
    let ks = keystages.choose(&mut rand::rng()).unwrap();

    let (numerator, denominator, answers, correct_answer_idx) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            1 => {
                let(n1, n2, ans) = generate_integer_division(&[2,5,10], 1..9);
                let (w, i) = generate_wrong_answers_int(ans.into(), params.answer_count, 1, 10);
                (n1, n2, w, i)
            },
            2 => {
                let(n1, n2, ans) = generate_integer_division(&[2,5,10], 1..15);
                let (w, i) = generate_wrong_answers_int(ans.into(), params.answer_count, 1, 15);
                (n1, n2, w, i)
            },
            3 => {
                let(n1, n2, ans) = generate_integer_division(&[2,5,10], 1..20);
                let (w, i) = generate_wrong_answers_int(ans.into(), params.answer_count, 1, 20);
                (n1, n2, w, i)
            },
            // TODO: use errors, not panics
            _ => panic!("division difficulty goes up to 3")
        },
        _ => panic!("Not yet implemented")
    };

    Question {
        text: format!("{numerator} ÷ {denominator} = ?").to_string(),
        answers: answers.iter().map(|n| n.to_string()).collect(),
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

// Unlike other strategies we don't use generics here, because the algorithm
// is different for different number types.
fn generate_integer_division(times_tables : &[u16], ans_range: Range<u16>) -> (u16, u16, u16) {
    let answer = rand::random_range(ans_range);
    let denominator = *times_tables.choose(&mut rand::rng()).unwrap();

    let numerator = answer * denominator;

    (numerator, denominator, answer)
}

/*
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

//fn generate_wrong_answers_float(
//    correct_answer: f32, count: u16, min_: f32, max_: f32, dp: u16) -> (Vec<f32>, usize) {
//}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_values() {
    }
}
