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

struct DayTwoSolution;

impl DayTwoSolution {
    pub fn is_invalid(id: u64) -> bool {
        let digit_count = u64::ilog10(id) + 1;

        if digit_count % 2 == 0 {
            let divisor = 10u64.pow(digit_count / 2);
            let top = id / divisor;
            let bottom = id - (top * divisor);

            log::trace!("{id}: {top}=={bottom}");
            return top == bottom;
        } else {
            false
        }
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



fn main() {
    init_logger();
    ProblemOne::solve::<DayTwoSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;
    use crate::{
        DayTwoSolution, 
        ProblemOne, 
        TEST_EXAMPLE
    };

    
    #[test]
    fn test_problem_one() {
        let result = ProblemOne::solve::<DayTwoSolution>(TEST_EXAMPLE);
        assert_eq!(result, 1227775554)
    }
}
