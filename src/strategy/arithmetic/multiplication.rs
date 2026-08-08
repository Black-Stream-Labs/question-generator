// Stage/level: (FIXME)
//   Key Stage 1 - 2, 5, 10 times tables
//      Difficulty 1 - up to N × 9
//                 2 - up to N × 15
//                 3 - up to N × 20
//                 4 - sometimes 3 numbers, up to ... × 9
//                 5 - sometimes 3 numbers, up to ... × 15
//   Key Stage 2 -
//   Key Stage 3 -
use rand::seq::{IndexedRandom};
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

pub fn generate_multiplication(params: &GeneratorParameters) -> Question {
    let keystages : Vec<KeyStage> = string_to_enum_vec(& params.curriculum.stage.clone().unwrap()).unwrap();
    let ks = keystages.choose(&mut rand::rng()).unwrap();

    let (nums, answers, correct_answer_idx) = match ks {
        KeyStage::Foundation => panic!("Foundation arithmetic not supported"),
        KeyStage::KeyStage1  => match params.curriculum.difficulty {
            // FIXME - wrong answer range should really be based on the actual answer
            1 => {
                let (n,a) = generate_multiplication_vals::<i32>(2, &[2,5,10], 0..9);
                let (w, i) = generate_wrong_answers_int(a, params.answer_count, 0, 90);
                (n,w,i)
            },
            2 => {
                let (n,a) = generate_multiplication_vals::<i32>(2, &[2,5,10], 0..15);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 150);
                (n,w,i)
            },
            3 => {
                let (n,a) = generate_multiplication_vals::<i32>(2, &[2,5,10], 0..20);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 200);
                (n,w,i)
            },
            4 => {
                let (n,a) = generate_multiplication_vals::<i32>(3, &[2,5,10], 0..9);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 90);
                (n,w,i)
            },
            5 => {
                let (n,a) = generate_multiplication_vals::<i32>(3, &[2,5,10], 0..15);
                let (w,i) = generate_wrong_answers_int(a, params.answer_count, 0, 150);
                (n,w,i)
            },
            // TODO: use errors, not panics
            _ => panic!("multiplication difficulty goes up to 5")
        },
        _ => panic!("Not yet implemented")
    };

    Question {
        text: (nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" × ") + " = ?"),
        answers: answers.iter().map(|n| n.to_string()).collect(),
        correct_answer: correct_answer_idx,
        explanation: None
    }
}

fn generate_multiplication_vals<T:Number>(
    num_nums: u16,
    times_tables: &[T],
    num_range: Range<T>,
)
    -> (Vec<T>, T) {
    let mut nums : Vec<T> = vec![];

    let mut correct_answer : T = T::from(1);
    // Each number except the last is from one of the requested times tables,
    // so produce n-1 numbers from that list and add a number from the range
    for _ in 1..num_nums {
        let n =  *times_tables.choose(&mut rand::rng()).unwrap();
        nums.push(n);
        correct_answer *= n;
    }
    {
        let n = rand::random_range(num_range);
        nums.push(n);
        correct_answer *= n;
    }

    // FIXME - don't understand borrowing enough to know why it complains about this
    // let correct_answer = nums.iter().reduce(|a,b| a * b).unwrap();

    (nums, correct_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_vals() {
    }
}
