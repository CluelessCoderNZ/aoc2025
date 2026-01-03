mod model;
mod parser;
use common::{Input, ProblemQuestion, Solution};

use crate::{model::Machine, parser::MachineManualParser};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;

impl ProblemQuestion for ProblemOne {
    type Parser = MachineManualParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = MachineManualParser;
    type Output = u64;
}

struct DayTenSolution;

impl Solution<ProblemOne> for DayTenSolution {
    fn answer(input: Vec<Machine>) -> u64 {
        input
            .into_iter()
            .map(|machine| machine.min_startup_presses())
            .sum()
    }
}

impl Solution<ProblemTwo> for DayTenSolution {
    fn answer(input: Vec<Machine>) -> u64 {
        input
            .into_iter()
            .map(|machine| machine.min_joltage_presses())
            .sum()
    }
}

fn main() {
    ProblemOne::solve::<DayTenSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayTenSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DayTenSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE};

    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayTenSolution>(TEST_EXAMPLE);
        assert_eq!(result, 7)
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DayTenSolution>(TEST_EXAMPLE);
        assert_eq!(result, 33)
    }
}
