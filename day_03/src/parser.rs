use common::LineInputParser;

use crate::model::BatteryBank;


pub struct BatteryBankParser;

impl LineInputParser for BatteryBankParser {
    type LineOutput = BatteryBank;

    fn parse_line(line: &str) -> Self::LineOutput {
        let batteries = line.chars()
            .map(|chr| chr.to_digit(10).expect("All characters are valid digits") as u8)
            .collect();

        BatteryBank {
            batteries
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{TEST_EXAMPLE, parser::BatteryBankParser};


    #[test]
    fn test_parser_ok() {
        let result = TEST_EXAMPLE.parse::<BatteryBankParser>();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_parser_line() {
        let result = TEST_EXAMPLE.parse::<BatteryBankParser>();

        let expected: [u8; 15] = [9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        assert_eq!(result[0].batteries, expected)
    }
}