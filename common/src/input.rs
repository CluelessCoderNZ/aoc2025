use std::fs;
use super::parser::InputParser;

pub struct Input(String);

impl Input {
    const TEST_PATH: &str = "input";

    pub fn from_test_file() -> Self {
        Self::from_file(Self::TEST_PATH)
    }

    pub fn from_file(path: &str) -> Self {
        let contents: String = fs::read_to_string(path)
            .expect("file is present");

        Self(contents)
    }

    pub fn from_str(str: &str) -> Self {
        Self(str.to_string())
    }
}

impl Input {
    pub fn parse<T: InputParser>(self) -> T::Output {
        T::parse_input(self.0.as_str())
    }
}