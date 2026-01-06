mod model;
mod parser;

use common::{Input, ProblemQuestion, Solution};

use crate::{model::PuzzleSummary, parser::PuzzleParser};


pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = PuzzleParser;
    type Output = u64;
}

struct DayTweleveSolution;

impl Solution<ProblemOne> for DayTweleveSolution {
    fn answer(input: PuzzleSummary) -> u64 {
        todo!()
    }
}


fn main() {
    println!("Hello, world!");
}
