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

    let (nums, ans) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            1 => generate_addition_vals::<i32>(2, 0..9, 0..9),
            2 => generate_addition_vals::<i32>(3, 0..9, 0..15),
            3 => generate_addition_vals::<i32>(3, 0..9, 0..30),
            4 => generate_addition_vals::<i32>(4, 0..9, 0..30),
            5 => generate_addition_vals::<i32>(4, 10..20, 20..80),
            _ => panic!("addition difficulty goes up to 5")
        },
        _ => panic!("Not yet implemented")
    };

    Question {
        text: "`num_1` + `num_2` = ?".to_string(),
        answers: nums.iter().map(|n| n.to_string()).collect(),
        correct_answer: 0,
        explanation: None
    }
}

fn generate_addition_vals<T: Number>(num_nums: u16, num_range: Range<T>, ans_range: Range<T>)
    -> (Vec<T>, T) {
    assert!(ans_range.start >= num_range.start * T::from(num_nums as i32),
        "Minimum answer must be at least `num_nums` * minimum number");

    let ans = rand::random_range(ans_range);
    let mut nums : Vec<T> = vec![];
    let mut running_total = ans;

    // We want to allow duplicates, so for N-1 numbers, pick a random number
    // less than the sum so far and add it to the list. Then for the Nth number
    // just use what's left.
    for _ in 1..num_nums {
        let this_num = rand::random_range(num_range.start..running_total);
        running_total -= this_num;
        nums.push(this_num);
    }
    if running_total > T::from(0) { nums.push(running_total) }

    (nums, ans)
}

// We have a different algorithm for generating floating point values, so we put the type in the
// function name, not in a template. We're not flexible on types here because we'll never be using
// this to produce mahoosive numbers ... right?
fn generate_wrong_answers_int(
    correct_answer: i32, count:u16, min_: i32, max_: i32) -> (Vec<i32>, usize) {
    let (min,max) = if min_ > max_ { (max_, min_) } else { (min_, max_) };

    let range = min..=max;
    assert!(range.end() - range.start() >= count.into(), "Spread must be at least as big as count!");

    let mut wrong_answers : Vec<i32> = range.collect();
    wrong_answers.retain(|x| *x != correct_answer);
    wrong_answers.shuffle(&mut rand::rng());

    let mut answers = wrong_answers[0..count as usize].to_vec();

    let correct_answer_idx : usize = rand::random_range(0..answers.len());
    answers.insert(correct_answer_idx, correct_answer);

    (answers, correct_answer_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_vals() {
        let (nums, ans) = generate_addition_vals::<i32>(2, 0..9, 2..9);
        assert_eq!(nums.len(), 2, "Got 2 numbers, as requested");
        assert_eq!(nums.iter().sum::<i32>(), ans, "They sum to the given answer");
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
