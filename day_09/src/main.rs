mod parser;
use common::{Box2D, Input, ProblemQuestion, Solution};
use parser::{Point2D, Point2DParser};
use itertools::Itertools;



pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = Point2DParser;
    type Output = usize;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = Point2DParser;
    type Output = usize;
}

struct DayNineSolution;

impl Solution<ProblemOne> for DayNineSolution {
    fn answer(input: Vec<Point2D>) -> usize {
        input.iter()
            .tuple_combinations::<(&Point2D, &Point2D)>()
            .map(|(a, b)| (a.x.abs_diff(b.x)+1)*(a.y.abs_diff(b.y)+1))
            .max().expect("At least two points")
    }
}

impl DayNineSolution {
    fn into_polygon(input: &Vec<Point2D>) -> Vec<(Point2D, Point2D)> {
        let mut output = Vec::with_capacity(input.len());
        for index in 0..input.len() {
            let a = input[index];
            let b = input[(index + 1) % input.len()];
            output.push((a, b));
        }

        output
    }

    fn area_inclusive(rect: &Box2D) -> usize {
        let size = rect.size();
        ((size.width + 1) * (size.height + 1)) as usize
    }
}

impl Solution<ProblemTwo> for DayNineSolution {
    fn answer(input: Vec<Point2D>) -> usize {
        let lines: Vec<Box2D> = Self::into_polygon(&input).into_iter()
            .map(|(a,b)| Box2D::from_points([a,b]))
            .collect();
        let rectangles = input.into_iter()
            .tuple_combinations()
            .map(|(a,b)| Box2D::from_points([a,b]));

        let valid_rectangles = rectangles.filter(|candidate| {
            // Note: This works since intersection excludes edges in euclid
            !lines.iter().any(|line| candidate.intersects(line))
        });

        let largest = valid_rectangles.max_by_key(|rect| Self::area_inclusive(rect))
            .expect("At least one valid rectangle");
        let area = Self::area_inclusive(&largest);

        println!(
            "Largest found: ({}, {}) -> ({}, {}) = {}",
            largest.min.x, largest.min.y,
            largest.max.x, largest.max.y,
            area
        );
        area
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