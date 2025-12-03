mod parser;
mod model;

use common::{Input, ProblemQuestion, Solution};

use crate::{model::BatteryBank, parser::BatteryBankParser};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = BatteryBankParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = BatteryBankParser;
    type Output = u64;
}

struct DayThreeSolution;

impl Solution<ProblemOne> for DayThreeSolution {
    fn answer(input: Vec<BatteryBank>) -> u64 {
        input.iter()
            .map(|bank| bank.max_joltage(2))
            .sum()
    }
}

impl Solution<ProblemTwo> for DayThreeSolution {
    fn answer(input: Vec<BatteryBank>) -> u64 {
        input.iter()
            .map(|bank| bank.max_joltage(12))
            .sum()
    }
}


fn main() {
    ProblemOne::solve::<DayThreeSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayThreeSolution>(TEST_INPUT);
}


#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DayThreeSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE};


    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayThreeSolution>(TEST_EXAMPLE);
        assert_eq!(result, 357)
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DayThreeSolution>(TEST_EXAMPLE);
        assert_eq!(result, 3121910778619)
    }
}