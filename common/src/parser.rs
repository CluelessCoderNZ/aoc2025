pub trait InputParser {
    type Output;

    fn parse_input(input: &str) -> Self::Output;
}

pub trait LineInputParser {
    type Output;

    fn parse_line(line: &str) -> Self::Output;
}

impl<T: LineInputParser> InputParser for T {
    type Output = Vec<T::Output>;

    fn parse_input(input: &str) -> Self::Output {
        input.lines().map(|line| Self::parse_line(line)).collect()
    }
}
