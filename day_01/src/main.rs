mod parser;
mod dial;

use common::{
    init_logger,
    Input, 
    ProblemQuestion, 
    Solution
};

use parser::DialInstructionParser;
use dial::{
    DialInstruction, 
    count_clicks, 
    count_zeros
};


const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = DialInstructionParser;
    type Output = usize;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = DialInstructionParser;
    type Output = i32;
}

struct DialPasswordSolution;
impl Solution<ProblemOne> for DialPasswordSolution {
    fn answer(input: Vec<DialInstruction>) -> usize {
        count_zeros(input)
    }
}

impl Solution<ProblemTwo> for DialPasswordSolution {
    fn answer(input: Vec<DialInstruction>) -> i32 {
        count_clicks(input)
    }
}


fn main() {
    init_logger();
    ProblemOne::solve::<DialPasswordSolution>(TEST_INPUT);
    ProblemTwo::solve::<DialPasswordSolution>(TEST_INPUT);
}


#[cfg(test)]
mod test {
    use common::{
        Input, 
        ProblemQuestion
    };
    use crate::{
        DialPasswordSolution, 
        ProblemOne, 
        ProblemTwo
    };

    const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));

    #[test]
    fn test_example_part1() {
        assert_eq!(
            ProblemOne::solve::<DialPasswordSolution>(TEST_EXAMPLE),
            3
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            ProblemTwo::solve::<DialPasswordSolution>(TEST_EXAMPLE),
            6
        );
    }
}