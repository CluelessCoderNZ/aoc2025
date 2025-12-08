mod model;
mod parser;
use common::{Grid2DWhitespaceParser, Input, ProblemQuestion, Solution};
use model::MathCell;

use crate::{model::{MathHomework, get_equations}, parser::MathHomeworkSpacePerservingParser};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

pub type MathHomeworkParser = Grid2DWhitespaceParser<MathCell>;

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = MathHomeworkSpacePerservingParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = MathHomeworkParser;
    type Output = u64;
}

struct DaySixSolution;

impl Solution<ProblemOne> for DaySixSolution {
    fn answer(input: MathHomework) -> u64 {
        let equations = get_equations(&input);
        equations.map(|(values,op)| {
            values.into_iter()
                .fold(op.identity(), |last, item| op.apply(last, item))
        }).sum()
    }
}

impl Solution<ProblemTwo> for DaySixSolution {
    fn answer(input: MathHomework) -> u64 {
        // let equations = get_digit_equations(&input);
        // equations.map(|(values,op)| {
        //     values.into_iter()
        //         .fold(op.identity(), |last, item| op.apply(last, item))
        // }).sum()
        unimplemented!()
    }
}


fn main() {
    ProblemOne::solve::<DaySixSolution>(TEST_INPUT);
    //ProblemTwo::solve::<DaySixSolution>(TEST_INPUT);
}


#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DaySixSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE};


    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DaySixSolution>(TEST_EXAMPLE);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DaySixSolution>(TEST_EXAMPLE);
        assert_eq!(result, 3263827);
    }
}