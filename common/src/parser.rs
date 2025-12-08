use std::{fmt::Debug, marker::PhantomData};
use nom::{
    IResult, 
    Parser, 
    character::complete::{char, digit1}, 
    combinator::map_res, 
    sequence::separated_pair
};
use std::{
    num::ParseIntError, 
    ops::RangeInclusive, 
    str::FromStr
};

pub trait InputParser {
    type Output;

    fn parse_input(input: &str) -> Self::Output;
}

pub trait LineInputParser {
    type LineOutput;

    fn parse_line(line: &str) -> Self::LineOutput;
}

impl<T: LineInputParser> InputParser for T {
    type Output = Vec<T::LineOutput>;

    fn parse_input(input: &str) -> Self::Output {
        input.lines().map(|line| Self::parse_line(line)).collect()
    }
}

pub struct CSVParser<T: FromStr> {
    _element: PhantomData<T>,
}

impl<T: FromStr> LineInputParser for CSVParser<T> where <T as FromStr>::Err: Debug {
    type LineOutput = Vec<T>;

    fn parse_line(line: &str) -> Self::LineOutput {
        line.split(',').map(|token| T::from_str(token).expect("Parse token ok")).collect()
    }
}


fn split_parse<'a, P: InputParser>(input: &'a str, delimiter: &str) -> Option<(P::Output, &'a str)> {
    let (parser_input, remainder) = input.split_once(delimiter)?;
    let parser_output = P::parse_input(parser_input);
    
    Some((parser_output, remainder))
}

pub struct DualSectionParser<A: InputParser, B: InputParser> {
    _parser_a: PhantomData<A>,
    _parser_b: PhantomData<B>
}

impl<A: InputParser, B: InputParser> InputParser for DualSectionParser<A, B> {
    type Output = (A::Output, B::Output);

    fn parse_input(input: &str) -> Self::Output {
        let (a_output, input) = split_parse::<A>(input, "\n\n")
            .expect("Can split parsing sections okay");
        let b_output = B::parse_input(input);

        (a_output, b_output)
    }
}

pub struct InclusiveRangeListParser;

impl LineInputParser for InclusiveRangeListParser {
    type LineOutput = RangeInclusive<u64>;

    fn parse_line(line: &str) -> Self::LineOutput {
        Self::parse_range(line).expect("Can parse input ok").1
    }
}

impl InclusiveRangeListParser {
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

pub struct FromStrParser<T: FromStr> {
    _element: PhantomData<T>
}

impl<T: FromStr> LineInputParser for FromStrParser<T> 
  where <T as FromStr>::Err: Debug
{
    type LineOutput = T;

    fn parse_line(line: &str) -> Self::LineOutput {
        T::from_str(line).expect("FromStr always succeeds")
    }
}