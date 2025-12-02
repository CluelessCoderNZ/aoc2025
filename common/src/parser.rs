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
