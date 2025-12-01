mod parser;
mod dial;
use common::Input;
use parser::DialInstructionParser;
use dial::{
    count_zeros,
    count_clicks
};

fn part_one() {
    let instructions = Input::from_test_file().parse::<DialInstructionParser>();
    let password = count_zeros(instructions);
    println!("Part 1 Password: {password}");
}

fn part_two() {
    let instructions = Input::from_test_file().parse::<DialInstructionParser>();
    let password = count_clicks(instructions);
    println!("Part 2 Password: {password}");
}

fn main() {
    part_one();
    part_two();
}


#[cfg(test)]
mod test {
    use common::Input;
    use crate::{
        dial::{
            DialInstruction, 
            count_clicks, 
            count_zeros
        }, 
        parser::DialInstructionParser
    };

    const EXAMPLE: &str =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_example_part1() {
        let instructions = Input::from_str(EXAMPLE).parse::<DialInstructionParser>();
        assert_eq!(count_zeros(instructions), 3);
    }

    #[test]
    fn test_example_part2() {
        let instructions = Input::from_str(EXAMPLE).parse::<DialInstructionParser>();

        assert_eq!(count_clicks(instructions), 6);
    }

    #[test]
    fn test_rollover() {
        let instructions = vec![
            DialInstruction::Left(1000)
        ];
        assert_eq!(count_clicks(instructions), 10)
    }

    #[test]
    fn test_rollover_remainder() {
        let instructions = vec![
            DialInstruction::Left(1050)
        ];
        assert_eq!(count_clicks(instructions), 11)
    }

    #[test]
    fn test_rollover_remainder_rollover() {
        let instructions = vec![
            DialInstruction::Left(1051)
        ];
        assert_eq!(count_clicks(instructions), 11)
    }
}