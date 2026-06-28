// Stage/level:
//   Key Stage 1 - positive integers and zero
//      Difficulty 1 - single-digit numbers
//                 2 - maybe double-digit numbers, single-digit answers
//                 3 - small double-digit numbers, maybe double-digit answers
//                 4 - medium double-digit numbers, small double-digit answers
//                 5 - any non-negative numbers below 100
//   Key Stage 2 -
//   Key Stage 3 -
use rand::seq::{IndexedRandom, SliceRandom};
use std::ops::Range;
use std::cmp::{PartialOrd, PartialEq, Ord, Eq};
use num_traits::{Num, NumAssignOps};
use super::generate_wrong_answers_int;

use crate::{
    Question,
    GeneratorParameters,
    KeyStage,
    string_to_enum_vec
};

trait Number: Num
    + NumAssignOps
    + PartialOrd
    + PartialEq
    + Ord
    + Eq
    + rand::distr::uniform::SampleUniform
    + From<i32>
    + Copy
{}

impl<T> Number for T where
T: Num
    + NumAssignOps
    + PartialOrd
    + PartialEq
    + Ord
    + Eq
    + rand::distr::uniform::SampleUniform
    + From<i32>
    + Copy
{}

pub fn generate_subtraction(params: &GeneratorParameters) -> Question {
    let keystages : Vec<KeyStage> = string_to_enum_vec(& params.curriculum.stage.clone().unwrap()).unwrap();
    let ks = keystages.choose(&mut rand::rng()).unwrap();

    let (nums, answers, correct_answer_idx) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            1 => {
                let (n,a) = generate_subtraction_vals::<i32>(2, 0..9, 0..4);
                let (w, i) = generate_wrong_answers_int(a, params.count, 0, 9);
                (n,w,i)
            },
            2 => {
                let (n,a) = generate_subtraction_vals::<i32>(3, 0..20, 0..9);
                let (w,i) = generate_wrong_answers_int(a, params.count, 0, 20);
                (n,w,i)
            },
            3 => {
                let (n,a) = generate_subtraction_vals::<i32>(2, 0..30, 0..15);
                let (w,i) = generate_wrong_answers_int(a, params.count, 0, 45);
                (n,w,i)
            },
            4 => {
                let (n,a) = generate_subtraction_vals::<i32>(4, 10..40, 0..20);
                let (w,i) = generate_wrong_answers_int(a, params.count, 0, 45);
                (n,w,i)
            },
            5 => {
                let (n,a) = generate_subtraction_vals::<i32>(2, 0..99, 0..80);
                let (w,i) = generate_wrong_answers_int(a, params.count, 0, 100);
                (n,w,i)
            },
            // TODO: use errors, not panics
            _ => panic!("subtraction difficulty goes up to 5")
        },
        _ => panic!("Not yet implemented")
    };

    Question {
        text: (nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" - ") + " = ?").to_string(),
        answers: answers.iter().map(|n| n.to_string()).collect(),
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

fn generate_subtraction_vals<T: Number>(num_nums: u16, num_range: Range<T>, ans_range: Range<T>)
    -> (Vec<T>, T) {
    assert!(num_range.end > ans_range.end,
        "Max of number range must be bigger than max of answer range");
    // This is harder than addition because you can't just pick a bunch of numbers in num_range and
    // add them together; each number needs to be smaller than the previous in a way that honours
    // the provided constraints. To make it easier I'm going to treat num_range as a guideline.
    // There's probably a cleaner way of doing this but let's just start with a number and add
    // numbers to it until we have either num_nums numbers or a number outside of num_range
    let ans = rand::random_range(ans_range);
    let mut nums : Vec<T> = vec![];
    let mut running_total = ans;

    for _ in 1..num_nums {
        // In theory this means that each number picked is bigger than the previous, so we put it
        // at the start of the vector and end up sorted with the biggest number first
        let r = running_total..num_range.end;
        if r.is_empty() { break }

        let this_num = rand::random_range(r);
        nums.insert(0, this_num);
        running_total += this_num;
    }
    // num_nums is a guideline but we need at least two, so create one from the difference between
    // ans and the existing number, which is probably zero if we got this far
    if nums.len() < 2 { nums.push( nums[0] - ans ) }

    (nums, ans)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction_vals() {
        let (nums, ans) = generate_subtraction_vals::<i32>(2, 0..9, 0..4);
        assert_eq!(nums.len(), 2, "Got 2 numbers, as requested");
        assert_eq!(nums.into_iter().reduce(|a,b| a - b), Some(ans), "Subtracted numbers equal answer");

        let (nums, ans) = generate_subtraction_vals::<i32>(5, 100..200, 10..20);
        assert!(nums.len() <= 5, "Got up to 5 numbers, as requested");
        assert_eq!(nums.into_iter().reduce(|a,b| a - b), Some(ans), "Subtracted numbers equal answer");
    }
}
