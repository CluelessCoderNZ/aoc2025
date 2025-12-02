use common::InputParser;
use nom::{
    IResult, 
    Parser, 
    character::complete::{char, digit1}, 
    combinator::map_res, 
    sequence::separated_pair,
    multi::separated_list0
};
use std::{
    num::ParseIntError, 
    ops::RangeInclusive, 
    str::FromStr
};


pub struct IdRangeListParser;

impl InputParser for IdRangeListParser {
    type Output = Vec<RangeInclusive<u64>>;

    fn parse_input(input: &str) -> Self::Output {
        separated_list0(char(','), Self::parse_range)
        .parse(input)
        .expect("Can parse input ok").1
    }
}

impl IdRangeListParser {
    fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
        map_res(
            separated_pair(digit1, char('-'), digit1),
            |(start_str, end_str)| {
                let start = u64::from_str(start_str)?;
                let end = u64::from_str(end_str)?;

                Result::<RangeInclusive<u64>, ParseIntError>::Ok(RangeInclusive::new(start, end))
            }
        ).parse(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        TEST_EXAMPLE, 
        parser::IdRangeListParser
    };

    #[test]
    fn test_parse_list_ok() {
        let result = TEST_EXAMPLE.parse::<IdRangeListParser>();
        assert_eq!(result.len(), 11);
    }

    #[test]
    fn test_parse_range_ok() {
        let result = TEST_EXAMPLE.parse::<IdRangeListParser>()
            .into_iter()
            .next()
            .expect("Has at least one element");
        assert_eq!(*result.start(), 11);
        assert_eq!(*result.end(), 22);
    }
}