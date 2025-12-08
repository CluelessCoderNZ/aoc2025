use std::str::FromStr;

use common::{Grid2D, InputParser};
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::{char, space0}, multi::many0, sequence::pair};

use crate::model::MathCell;


pub struct MathHomeworkSpacePerservingParser;

impl MathHomeworkSpacePerservingParser {
    fn split_operators(line: &str) -> Vec<String> {
        let output = many0(Self::operator_token)
            .parse(line).expect("Can parse op line ok");

        output.1
    }

    fn operator_token(input: &str) -> IResult<&str, String> {
        let (input, token) = alt((
            pair(tag("+"), space0),
            pair(tag("*"), space0)
        )).parse(input)?;

        let token = format!("{}{}", token.0, token.1);

        Result::Ok((input, token))
    }

    // Uhh this is a bit of a mess but the purpose is to ensure all value strings
    // are algined vertically with their characters
    fn split_value_lines(operators: Vec<String>, lines: Vec<&str>) -> Vec<Vec<MathCell>> {
        let mut output = Vec::with_capacity(lines.len());
        
        for line in lines {
            let mut gird_line = Vec::with_capacity(operators.len());
            let mut token_start = 0;
            for op_str in &operators {
                let token_end = token_start + op_str.len();
                let token = &line[token_start..token_end];
                let cell = MathCell::from_str(token).expect("Parse cell ok");
                gird_line.push(cell);

                token_start = token_end;
            }

            output.push(gird_line);
        }
        
        let op_line = operators.into_iter()
            .map(|op| MathCell::from_str(&op).expect("Parse op okay"))
            .collect();

        output.push(op_line);

        output
    }
}

impl InputParser for MathHomeworkSpacePerservingParser
{
    type Output = Grid2D<MathCell>;

    fn parse_input(input: &str) -> Self::Output {
        let mut lines: Vec<&str> = input.lines().collect();

        let operator_line = lines.pop().expect("At least one line");

        let operators = Self::split_operators(operator_line);

        let grid_lines = Self::split_value_lines(operators, lines);

        let height = grid_lines.len() as isize;
        let width = grid_lines.get(0).expect("At least one line of input").len() as isize;
        let elements = grid_lines.into_iter().flatten().collect();

        Self::Output {
            elements,
            width,
            height
        }
    }
}

#[cfg(test)]
mod test {
    use core::num;

    use common::{Grid2D, InputParser, Point2D};

    use crate::{TEST_EXAMPLE, model::MathCell, parser::MathHomeworkSpacePerservingParser};

    fn assert_str(grid: &Grid2D<MathCell>, x: isize, y: isize, pattern: &str) {
        let item = grid.get(Point2D::new(x, y));
        if let Some(MathCell::Number(_, num_str)) = item {
            assert_eq!(num_str, pattern);
        } else { 
            panic!("Expected cell to be number {x}, {y}: {item:?}");
        }
    }


    #[test]
    fn test_space_perserving_parser() {
        let grid = TEST_EXAMPLE.parse::<MathHomeworkSpacePerservingParser>();
        assert_str(&grid, 0, 0, "123 ");
        assert_str(&grid, 1, 0, "328 ");
        assert_str(&grid, 2, 0, " 51 ");
        assert_str(&grid, 3, 0, "64 ");

        assert_str(&grid, 0, 1, " 45 ");
        assert_str(&grid, 1, 1, "64  ");
        assert_str(&grid, 2, 1, "387 ");
        assert_str(&grid, 3, 1, "23 ");

        assert_str(&grid, 0, 2, "  6 ");
        assert_str(&grid, 1, 2, "98  ");
        assert_str(&grid, 2, 2, "215 ");
        assert_str(&grid, 3, 2, "314");
    }
}