// Stage/level:
//   Key Stage 1 - positive integers and zero
//      Difficulty 1 - single-digit answers
//                 2 - up to 3 numbers, answers up to 15
//                 3 - up to 3 numbers, answers up to 30
//                 4 - up to 4 numbers, answers up to 30
//                 5 - up to 4 numbers, all double-digit
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

trait Number: Num
    + NumAssignOps
    + PartialOrd
    + PartialEq
    + rand::distr::uniform::SampleUniform
    + From<i32>
    + Copy
{}

impl<T> Number for T where
T: Num
    + NumAssignOps
    + PartialOrd
    + PartialEq
    + rand::distr::uniform::SampleUniform
    + From<i32>
    + Copy
{}

pub fn generate_addition(params: &GeneratorParameters) -> Question {
    let keystages : Vec<KeyStage> = string_to_enum_vec(& params.curriculum.stage.clone().unwrap()).unwrap();
    let ks = keystages.choose(&mut rand::rng()).unwrap();

    let (nums, answers, correct_answer_idx) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            1 => {
                let (n,a) = generate_addition_vals::<i32>(2, 0..9, 0..9);
                let (w, i) = generate_wrong_answers_int(a, params.answer_count, 0, 9);
                (n,w,i)
            },
            2 => {
                let (n,a) = generate_addition_vals::<i32>(3, 0..9, 0..15);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 20);
                (n,w,i)
            },
            3 => {
                let (n,a) = generate_addition_vals::<i32>(2, 0..9, 0..30);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 45);
                (n,w,i)
            },
            4 => {
                let (n,a) = generate_addition_vals::<i32>(4, 0..9, 0..30);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 45);
                (n,w,i)
            },
            5 => {
                let (n,a) = generate_addition_vals::<i32>(2, 10..20, 20..80);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 100);
                (n,w,i)
            },
            // TODO: use errors, not panics
            _ => panic!("addition difficulty goes up to 5")
        },
        _ => panic!("Not yet implemented")
    };

    Question {
        text: (nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + ") + " = ?").to_string(),
        answers: answers.iter().map(|n| n.to_string()).collect(),
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

fn generate_addition_vals<T: Number>(num_nums: u16, num_range: Range<T>, ans_range: Range<T>)
    -> (Vec<T>, T) {
    assert!(ans_range.start >= num_range.start * T::from(i32::from(num_nums)),
        "Minimum answer must be at least `num_nums` * minimum number");

    let ans = rand::random_range(ans_range);

    // Algorithm doesn't like picking a number from 0..0 N times
    if ans == T::from(0) {
        return (vec![0.into(); num_nums.into()], 0.into());
    }

    let mut nums : Vec<T> = vec![];
    let mut running_total = ans;

    // We want to allow duplicates, so for N-1 numbers, pick a random number
    // less than the sum so far and add it to the list. Then for the Nth number
    // just use what's left.
    for _ in 1..num_nums {
        let r = num_range.start..running_total;
        if r.is_empty() { break }

        let this_num = rand::random_range(r);
        running_total -= this_num;
        nums.push(this_num);
    }
    if running_total > T::from(0) { nums.push(running_total) }

    (nums, ans)
}

//fn generate_wrong_answers_float(
//    correct_answer: f32, count: u16, min_: f32, max_: f32, dp: u16) -> (Vec<f32>, usize) {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_vals() {
        let (nums, ans) = generate_addition_vals::<i32>(2, 0..9, 2..9);
        assert_eq!(nums.len(), 2, "Got 2 numbers, as requested");
        assert_eq!(nums.iter().sum::<i32>(), ans, "They sum to the given answer");

        let (nums, ans) = generate_addition_vals::<i32>(5, 10..20, 100..200);
        assert!(nums.len() <= 5, "Got up to 5 numbers, as requested");
        assert_eq!(nums.iter().sum::<i32>(), ans, "They sum to the given answer");

        let (nums, _) = generate_addition_vals::<i32>(5, 0..20, 0..0);
        assert_eq!(nums.len(), 5, "Got 5 numbers, as requested");
        assert_eq!(nums, vec![0;5]);
    }

    #[test]
    fn wrong_answers_int() {
        let (answers, index) = generate_wrong_answers_int(5, 5, 0, 10);
        assert!(answers.clone().into_iter().filter(|n| *n < 0).collect::<Vec<_>>().is_empty(),
            "All values min 0");
        assert!(answers.clone().into_iter().filter(|n| *n > 10).collect::<Vec<_>>().is_empty(),
            "All values max 10");
        assert_eq!(answers[index], 5, "Answer in the correct index");

        let (answers, index) = generate_wrong_answers_int(5, 5, -10, 10);
        assert!(answers.clone().into_iter().filter(|n| *n < -10).collect::<Vec<_>>().is_empty(),
            "All values min -10");
        assert!(answers.clone().into_iter().filter(|n| *n > 10).collect::<Vec<_>>().is_empty(),
            "All values max 10");
        assert_eq!(answers[index], 5, "Answer in the correct index");
    }
}
