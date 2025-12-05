use std::{collections::HashSet, ops::RangeInclusive};

use common::{
    DualSectionParser, FromStrParser, InclusiveRangeListParser, Input, ProblemQuestion, Solution,
};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

type IngredientListParser = DualSectionParser<InclusiveRangeListParser, FromStrParser<u64>>;

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = IngredientListParser;
    type Output = usize;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = IngredientListParser;
    type Output = usize;
}

struct DayFiveSolution;

impl DayFiveSolution {
    fn does_overlap(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
        a.contains(b.start()) || a.contains(b.end())
    }

    fn try_merge(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
        if Self::does_overlap(&a, &b) {
            Some(RangeInclusive::new(
                a.start().min(b.start()).clone(),
                a.end().max(b.end()).clone(),
            ))
        } else {
            None
        }
    }

    fn merge_all(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        ranges.sort_by_key(|range| *range.start());

        let mut i = 0;
        while i < ranges.len() - 1 {
            let a = &ranges[i];
            let b = &ranges[i + 1];

            match Self::try_merge(a, b) {
                Some(combined) => {
                    ranges[i] = combined;
                    ranges.remove(i + 1);
                }
                None => {
                    i += 1;
                }
            }
        }

        ranges
    }
}

impl Solution<ProblemOne> for DayFiveSolution {
    fn answer(input: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
        input
            .1
            .into_iter()
            .filter(|id| input.0.iter().any(|range| range.contains(id)))
            .count()
    }
}

impl Solution<ProblemTwo> for DayFiveSolution {
    fn answer(input: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
        Self::merge_all(input.0)
            .into_iter()
            .map(|range| range.count())
            .sum()
    }
}

pub struct DayFiveDumbSolution;
impl Solution<ProblemTwo> for DayFiveDumbSolution {
    fn answer(input: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
        let ranges = input.0;

        let ids = ranges.into_iter().flat_map(|range| range.into_iter());

        // Will eat all your ram
        let set = HashSet::<u64>::from_iter(ids);

        set.len()
    }
}

fn main() {
    ProblemOne::solve::<DayFiveSolution>(TEST_INPUT);
    ProblemTwo::solve::<DayFiveSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::{Input, ProblemQuestion};

    use crate::{
        DayFiveDumbSolution, DayFiveSolution, IngredientListParser, ProblemOne, ProblemTwo,
        TEST_EXAMPLE,
    };

    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayFiveSolution>(TEST_EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DayFiveSolution>(TEST_EXAMPLE);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_problem_two_example_dumb() {
        let result = ProblemTwo::solve::<DayFiveDumbSolution>(TEST_EXAMPLE);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_parser() {
        let (ranges, ids) = Input::parse::<IngredientListParser>(TEST_EXAMPLE);
        assert_eq!(ranges.len(), 4);
        assert_eq!(ids.len(), 6);

        assert_eq!(*ranges[0].start(), 3);
        assert_eq!(*ranges[0].end(), 5);
        assert_eq!(ids[0], 1);
        assert_eq!(ids[2], 8);
    }
}
