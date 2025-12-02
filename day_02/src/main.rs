mod parser;

use std::ops::RangeInclusive;

use common::{
    init_logger,
    Input, 
    ProblemQuestion, 
    Solution
};
use parser::IdRangeListParser;

const TEST_INPUT: Input = Input::from_str(include_str!("../input"));
const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = IdRangeListParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = IdRangeListParser;
    type Output = u64;
}

struct DayTwoSolution;

impl DayTwoSolution {
    pub fn is_invalid(id: u64) -> bool {
        let digit_count = u64::ilog10(id) + 1;

        if digit_count % 2 == 0 {
            let divisor = 10u64.pow(digit_count / 2);
            let top = id / divisor;
            let bottom = id - (top * divisor);

            //log::trace!("{id}: {top}=={bottom}");
            return top == bottom;
        } else {
            false
        }
    }

    pub fn strict_is_invalid(id: u64) -> bool {
        let digit_count = u64::ilog10(id) + 1;

        if digit_count < 2 {
            return false;
        }

        // Sequence chunk sizes which evenly divide number
        let seq_sizes = (1..=digit_count/2)
            .filter(|size| digit_count % size == 0);
        
        // Does any sequence size repeat across whole number
        seq_sizes.into_iter()
            .any(|size| Self::are_n_digits_repeated(id.clone(), size))
    }

    fn are_n_digits_repeated(mut value: u64, n: u32) -> bool {
        let mut last_pattern: Option<u64> = None;
        
        // Get last n digits of number check if matches pattern last time and continue
        while value != 0 {
            let divisor = 10u64.pow(n);
            let pattern = value - ((value / divisor) * divisor);
            //log::trace!("{value}[:{n}] = {pattern}");
            
            if last_pattern.is_some() && last_pattern != Some(pattern) {
                return false;
            }

            last_pattern = Some(pattern);
            value = value / divisor;
        }

        // If we finished then the pattern always matched
        return true;
    }

}

impl Solution<ProblemOne> for DayTwoSolution {
    fn answer(input: Vec<RangeInclusive<u64>>) -> u64 {
        input.into_iter()
        .flat_map(|range| range)
        .filter(|id| Self::is_invalid(*id))
        .sum()
    }
}

impl Solution<ProblemTwo> for DayTwoSolution {
    fn answer(input: Vec<RangeInclusive<u64>>) -> u64 {
        input.into_iter()
        .flat_map(|range| range)
        .filter(|id| Self::strict_is_invalid(*id))
        .sum()
    }
}


fn main() {
    init_logger();
    ProblemOne::solve::<DayTwoSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayTwoSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;
    use crate::{
        DayTwoSolution, 
        ProblemOne, 
        ProblemTwo, 
        TEST_EXAMPLE
    };

    
    #[test]
    fn test_problem_one() {
        let result = ProblemOne::solve::<DayTwoSolution>(TEST_EXAMPLE);
        assert_eq!(result, 1227775554)
    }

    #[test]
    fn test_problem_two() {
        let result = ProblemTwo::solve::<DayTwoSolution>(TEST_EXAMPLE);
        assert_eq!(result, 4174379265)
    }

    #[test]
    fn test_strict_ok() {
        assert_eq!(DayTwoSolution::strict_is_invalid(12_12_12_12), true);
        assert_eq!(DayTwoSolution::strict_is_invalid(1_1_1_1_1_1_1_1_1), true);
        assert_eq!(DayTwoSolution::strict_is_invalid(123_123_123_123), true);
        assert_eq!(DayTwoSolution::strict_is_invalid(1234_1234_1234), true);
        assert_eq!(DayTwoSolution::strict_is_invalid(1234_1235_1234), false);
        assert_eq!(DayTwoSolution::strict_is_invalid(123456789), false);
        assert_eq!(DayTwoSolution::strict_is_invalid(22), true);
        assert_eq!(DayTwoSolution::strict_is_invalid(2), false);
    }
}
