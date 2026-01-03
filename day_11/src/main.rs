mod model;
mod parser;

use std::collections::HashMap;

use common::{Input, ProblemQuestion, Solution};

use crate::{model::{Device, DeviceId, count_all_paths, count_problematic_paths}, parser::DeviceParser};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_EXAMPLE_TWO: Input = Input::from_str(include_str!("../example_two"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = DeviceParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = DeviceParser;
    type Output = u64;
}


struct DayElevenSolution;

impl Solution<ProblemOne> for DayElevenSolution {
    fn answer(input: Vec<Device>) -> u64 {
        let graph: HashMap<DeviceId, Vec<DeviceId>> = input.into_iter()
            .map(|dev| (dev.id, dev.outputs))
            .collect();

        count_all_paths(&graph)
    }
}

impl Solution<ProblemTwo> for DayElevenSolution {
    fn answer(input: Vec<Device>) -> u64 {
        let graph: HashMap<DeviceId, Vec<DeviceId>> = input.into_iter()
            .map(|dev| (dev.id, dev.outputs))
            .collect();

        count_problematic_paths(&graph)
    }
}

fn main() {
    ProblemOne::solve::<DayElevenSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayElevenSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DayElevenSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE, TEST_EXAMPLE_TWO};

    #[test]
    fn test_example_problem_one() {
        let result = ProblemOne::solve::<DayElevenSolution>(TEST_EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example_problem_two() {
        let result = ProblemTwo::solve::<DayElevenSolution>(TEST_EXAMPLE_TWO);
        assert_eq!(result, 2);
    }
}