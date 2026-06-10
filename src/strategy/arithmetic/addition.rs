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
    + From<usize>
    + Copy
{}

impl<T> Number for T where
T: Num
    + NumAssignOps
    + PartialOrd
    + PartialEq
    + rand::distr::uniform::SampleUniform
    + From<usize>
    + Copy
{}

pub fn generate_addition(params: &GeneratorParameters) -> Question {
    let keystages : Vec<KeyStage> = string_to_enum_vec(& params.curriculum.stage.clone().unwrap()).unwrap();
    let ks = keystages.choose(&mut rand::rng()).unwrap();

    let (nums, ans) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            1 => generate_addition_vals::<usize>(2, 0..9, 0..9),
            2 => generate_addition_vals::<usize>(3, 0..9, 0..15),
            3 => generate_addition_vals::<usize>(3, 0..9, 0..30),
            4 => generate_addition_vals::<usize>(4, 0..9, 0..30),
            5 => generate_addition_vals::<usize>(4, 10..20, 20..80),
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

fn generate_addition_vals<T: Number>(num_nums: usize, num_range: Range<T>, ans_range: Range<T>)
    -> (Vec<T>, T) {
    assert!(ans_range.start >= num_range.start * T::from(num_nums),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_vals() {
        let (nums, ans) = generate_addition_vals(2, 0..9, 2..9);
        println!("{:?} {:?}", nums, ans);
        assert_eq!(nums.len(), 2);
        assert_eq!(nums.iter().sum::<usize>(), ans);
    }
}
