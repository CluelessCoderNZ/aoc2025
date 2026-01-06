use common::{Grid2D, Grid2DParser, InputParser};

use crate::model::{PresentCell, PuzzleSummary, TreeRegion};


pub struct PuzzleParser;

impl PuzzleParser {
    fn parse_shape(input: &str) -> Grid2D<PresentCell> {
        let (_index_line, input) = input.split_once("\n").unwrap();
        Grid2DParser::parse_input(input)
    }

    fn parse_region(input: &str) -> TreeRegion {
        let (size_str, counts_str) = input.split_once(':').unwrap();
        
        let (x_str, y_str) = size_str.split_once('x').unwrap();
        let width: isize = x_str.parse().unwrap();
        let height: isize = y_str.parse().unwrap();

        let present_counts = counts_str
            .trim().split(" ")
            .map(|str| str.parse::<usize>().unwrap())
            .collect();

        TreeRegion { 
            width, 
            height, 
            present_counts
        }
    }
}

impl InputParser for PuzzleParser {
    type Output = PuzzleSummary;

    fn parse_input(input: &str) -> Self::Output {
        let sections: Vec<&str> = input.split("\n\n").collect();
        let (regions_str, presents_strs) = sections.split_last().expect("At least one element");

        let present_shapes = presents_strs.into_iter()
            .map(|input: &&str| Self::parse_shape(*input))
            .collect();

        let regions = regions_str.lines()
            .map(Self::parse_region)
            .collect();

        PuzzleSummary {
            present_shapes,
            regions,
        }
    }
}