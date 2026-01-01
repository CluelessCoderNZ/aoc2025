use common::{CSVParser, InputParser};

pub type Point2D = common::Point2D;

pub struct Point2DParser;

impl InputParser for Point2DParser {
    type Output = Vec<Point2D>;

    fn parse_input(input: &str) -> Self::Output {
        CSVParser::parse_input(input)
            .into_iter()
            .map(|pair: Vec<isize> | {
                let mut pair = pair.into_iter();
                let x = pair.next().unwrap();
                let y = pair.next().unwrap();
                
                Point2D::new(x, y)
            })
            .collect()
    }
}