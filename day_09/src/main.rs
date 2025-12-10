mod parser;
use common::{Input, ProblemQuestion, Solution};
use euclid::default::Box2D;
use parser::{Point2D, Point2DParser};
use itertools::Itertools;


pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = Point2DParser;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = Point2DParser;
    type Output = u64;
}

struct DayNineSolution;

impl Solution<ProblemOne> for DayNineSolution {
    fn answer(input: Vec<Point2D>) -> u64 {
        input.iter()
            .tuple_combinations::<(&Point2D, &Point2D)>()
            .map(|(a, b)| (a.x.abs_diff(b.x)+1)*(a.y.abs_diff(b.y)+1))
            .max().expect("At least two points")
    }
}

impl Solution<ProblemTwo> for DayNineSolution {
    fn answer(input: Vec<Point2D>) -> u64 {
        todo!()
    }
}

fn main() {
    ProblemOne::solve::<DayNineSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayNineSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;
    use crate::{DayNineSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE};


    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayNineSolution>(TEST_EXAMPLE);
        assert_eq!(result, 50)
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DayNineSolution>(TEST_EXAMPLE);
        assert_eq!(result, 24)
    }
}