use common::{CSVParser, InputParser};


pub type Point3D = euclid::default::Point3D<i64>;

pub struct Point3DParser;

impl InputParser for Point3DParser {
    type Output = Vec<Point3D>;

    fn parse_input(input: &str) -> Self::Output {
        CSVParser::parse_input(input)
            .into_iter()
            .map(|triplet: Vec<i64> | {
                let mut triplet = triplet.into_iter();
                let x = triplet.next().unwrap();
                let y = triplet.next().unwrap();
                let z = triplet.next().unwrap();
                
                Point3D::new(x, y, z)
            })
            .collect()
    }
}