use common::LineInputParser;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, space1},
    combinator::{map, map_res},
    multi::{many0, separated_list0},
    sequence::{delimited},
};

use crate::model::{Button, Machine};

pub struct MachineManualParser;

impl MachineManualParser {
    fn indicator_pattern(input: &str) -> IResult<&str, Vec<bool>> {
        delimited(
            char('['),
            many0(alt((map(char('.'), |_| false), map(char('#'), |_| true)))),
            char(']'),
        )
        .parse(input)
    }

    fn button(input: &str) -> IResult<&str, Button> {
        delimited(
            char('('),
            map(
                separated_list0(char(','), map_res(digit1, |s: &str| s.parse::<usize>())),
                |toggle_pattern| Button { toggle_pattern },
            ),
            char(')'),
        )
        .parse(input)
    }

    fn button_list(input: &str) -> IResult<&str, Vec<Button>> {
        delimited(space1, separated_list0(space1, Self::button), space1).parse(input)
    }

    fn joltage_req(input: &str) -> IResult<&str, Vec<u64>> {
        delimited(
            char('{'),
            separated_list0(char(','), map_res(digit1, |s: &str| s.parse::<u64>())),
            char('}'),
        )
        .parse(input)
    }

    fn machine_description(input: &str) -> IResult<&str, Machine> {
        let (input, indicator_pattern) = Self::indicator_pattern(input)?;
        let (input, button_wiring) = Self::button_list(input)?;
        let (input, joltage_req) = Self::joltage_req(input)?;

        let machine = Machine {
            indicator_pattern,
            button_wiring,
            joltage_req,
        };

        Ok((input, machine))
    }
}

impl LineInputParser for MachineManualParser {
    type LineOutput = Machine;

    fn parse_line(line: &str) -> Self::LineOutput {
        let (_, machine) = Self::machine_description(line).expect("input is correct syntax");
        machine
    }
}

#[cfg(test)]
mod test {
    use crate::{TEST_EXAMPLE, parser::MachineManualParser};

    #[test]
    fn test_example_parses_ok() {
        let output = TEST_EXAMPLE.parse::<MachineManualParser>();

        assert_eq!(output.len(), 3);
    }

    #[test]
    fn test_example_parses_indicators() {
        let output = TEST_EXAMPLE.parse::<MachineManualParser>();

        assert_eq!(output[0].indicator_pattern.len(), 4);
        assert_eq!(output[1].indicator_pattern.len(), 5);
        assert_eq!(output[2].indicator_pattern.len(), 6);
        assert_eq!(
            output[1].indicator_pattern,
            &[false, false, false, true, false]
        );
    }

    #[test]
    fn test_example_parses_buttons() {
        let output = TEST_EXAMPLE.parse::<MachineManualParser>();

        assert_eq!(output[0].button_wiring.len(), 6);
        assert_eq!(output[1].button_wiring.len(), 5);
        assert_eq!(output[2].button_wiring.len(), 4);
        assert_eq!(output[2].button_wiring[2].toggle_pattern, &[0, 1, 2, 4, 5]);
    }

    #[test]
    fn test_example_parses_joltage() {
        let output = TEST_EXAMPLE.parse::<MachineManualParser>();

        assert_eq!(output[0].joltage_req.len(), 4);
        assert_eq!(output[1].joltage_req.len(), 5);
        assert_eq!(output[2].joltage_req.len(), 6);
        assert_eq!(output[0].joltage_req, &[3, 5, 4, 7]);
    }
}
