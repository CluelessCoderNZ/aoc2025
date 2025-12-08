use common::{
    CardinalDirection, Direction, Grid2D, Grid2DParser, Input, Point2D, ProblemQuestion, Solution,
};

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramCell {
    Start,
    Splitter,
    SolvedSplitter(u64),
    Beam,
    Empty,
}

impl TryFrom<char> for DiagramCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            '|' => Ok(Self::Beam),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for DiagramCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            Self::Start => 'S',
            Self::Splitter => '^',
            Self::SolvedSplitter(_) => '!',
            Self::Beam => '|',
            Self::Empty => '.',
        };

        write!(f, "{chr}")
    }
}

fn count_splits_recursive(input: &mut Grid2D<DiagramCell>, beam_point: Point2D) -> u64 {
    const BEAM_DIR: CardinalDirection = CardinalDirection::S;
    let cell = input.get(beam_point).cloned();

    match cell {
        Some(DiagramCell::Splitter) => {
            1 + count_splits_recursive(input, beam_point + BEAM_DIR.rotate().delta())
                + count_splits_recursive(input, beam_point + BEAM_DIR.rotate_ccw().delta())
        }
        Some(DiagramCell::Start) => count_splits_recursive(input, beam_point + BEAM_DIR.delta()),
        Some(DiagramCell::Empty) => {
            let cell_mut = input.get_mut(beam_point).unwrap();
            *cell_mut = DiagramCell::Beam;

            count_splits_recursive(input, beam_point + BEAM_DIR.delta())
        }
        _ => 0,
    }
}

fn count_timelines_recursive(input: &mut Grid2D<DiagramCell>, beam_point: Point2D) -> u64 {
    const BEAM_DIR: CardinalDirection = CardinalDirection::S;
    let cell = input.get(beam_point).cloned();

    match cell {
        // Wow I got to say rust makes this behaviour read so well
        Some(DiagramCell::SolvedSplitter(val)) => val,
        Some(DiagramCell::Splitter) => {
            let result = count_timelines_recursive(input, beam_point + BEAM_DIR.rotate().delta())
                + count_timelines_recursive(input, beam_point + BEAM_DIR.rotate_ccw().delta());

            let cell_mut = input.get_mut(beam_point).unwrap();
            *cell_mut = DiagramCell::SolvedSplitter(result);

            result
        }
        Some(DiagramCell::Start) | Some(DiagramCell::Empty)
            => count_timelines_recursive(input, beam_point + BEAM_DIR.delta()),
        _ => 1,
    }
}

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = Grid2DParser<DiagramCell>;
    type Output = u64;
}

struct ProblemTwo;
impl ProblemQuestion for ProblemTwo {
    type Parser = Grid2DParser<DiagramCell>;
    type Output = u64;
}

struct DaySevenSolution;

impl Solution<ProblemOne> for DaySevenSolution {
    fn answer(mut input: Grid2D<DiagramCell>) -> u64 {
        let start_pos = input
            .element_iter_filtered(&DiagramCell::Start)
            .map(|(_, pos)| pos)
            .next()
            .expect("starting position");

        count_splits_recursive(&mut input, start_pos)
    }
}

impl Solution<ProblemTwo> for DaySevenSolution {
    fn answer(mut input: Grid2D<DiagramCell>) -> u64 {
        let start_pos = input
            .element_iter_filtered(&DiagramCell::Start)
            .map(|(_, pos)| pos)
            .next()
            .expect("starting position");

        count_timelines_recursive(&mut input, start_pos)
    }
}

fn main() {
    ProblemOne::solve::<DaySevenSolution>(TEST_INPUT);
    ProblemTwo::solve::<DaySevenSolution>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DaySevenSolution, ProblemOne, ProblemTwo, TEST_EXAMPLE};

    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DaySevenSolution>(TEST_EXAMPLE);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_problem_two_example() {
        let result = ProblemTwo::solve::<DaySevenSolution>(TEST_EXAMPLE);
        assert_eq!(result, 40);
    }
}
