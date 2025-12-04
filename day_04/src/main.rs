mod model;

use common::{
    Grid2D, 
    Grid2DParser, 
    Input, 
    ProblemQuestion, 
    Solution
};

use crate::model::{
    RoomCell, 
    count_available_rolls, 
    count_available_rolls_iterative
};


pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = Grid2DParser<RoomCell>;
    type Output = usize;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = Grid2DParser<RoomCell>;
    type Output = usize;
}

struct DayFourSolution;

impl Solution<ProblemOne> for DayFourSolution {
    fn answer(input: Grid2D<RoomCell>) -> usize {
        count_available_rolls(&input)
    }
}

impl Solution<ProblemTwo> for DayFourSolution {
    fn answer(mut input: Grid2D<RoomCell>) -> usize {
        count_available_rolls_iterative(&mut input)
    }
}


fn main() {
    ProblemOne::solve::<DayFourSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayFourSolution>(TEST_INPUT);
}


#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{
        DayFourSolution, 
        ProblemOne, 
        ProblemTwo, 
        TEST_EXAMPLE
    };

    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayFourSolution>(TEST_EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DayFourSolution>(TEST_EXAMPLE);
        assert_eq!(result, 43);
    }
}