use super::dial::DialInstruction;
use common::LineInputParser;
use std::str::FromStr;
use nom::{
    IResult, 
    Parser, 
    branch::alt, 
    character::complete::{char, digit1}, 
    combinator::map_res, 
    sequence::preceded
};

pub struct DialInstructionParser;

impl LineInputParser for DialInstructionParser {
    type LineOutput = DialInstruction;

    fn parse_line(line: &str) -> Self::LineOutput {
        alt((Self::parse_left, Self::parse_right))
        .parse(line)
        .expect("Line parses ok")
        .1
    }
}

impl DialInstructionParser {
    fn parse_left(input: &str) -> IResult<&str, DialInstruction> {
        map_res(
            preceded(char('L'), digit1), 
            |s| i32::from_str(s).map(|val|DialInstruction::Left(val))
        ).parse_complete(input)
    }

    fn parse_right(input: &str) -> IResult<&str, DialInstruction> {
        map_res(
            preceded(char('R'), digit1), 
            |s| i32::from_str(s).map(|val|DialInstruction::Right(val))
        ).parse_complete(input)
    }
}