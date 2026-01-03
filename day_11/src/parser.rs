use common::LineInputParser;

use crate::model::Device;

pub struct DeviceParser;

impl LineInputParser for DeviceParser {
    type LineOutput = Device;

    fn parse_line(line: &str) -> Self::LineOutput {
        let (name, outputs_str) = line.split_once(":").expect("valid syntax");
        let outputs = outputs_str.trim().split_whitespace().collect();
        Device::from_str(name, outputs).expect("valid string ids")
    }
}

#[cfg(test)]
mod test {
    use crate::{TEST_EXAMPLE, model::DeviceId, parser::DeviceParser};


    #[test]
    fn test_parse_example() {
        let devices = TEST_EXAMPLE.parse::<DeviceParser>();

        assert_eq!(devices.len(), 10);
        assert_eq!(devices[0].outputs.len(), 2);

        assert_eq!(devices[0].id, DeviceId::try_from("aaa").unwrap());
        assert_eq!(devices[0].outputs[0], DeviceId::try_from("you").unwrap());
        assert_eq!(devices[0].outputs[1], DeviceId::try_from("hhh").unwrap());
    }
}