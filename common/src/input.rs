use super::parser::InputParser;

#[derive(Clone)]
pub struct Input(&'static str);

impl Input {
    pub const fn from_str(str: &'static str) -> Self {
        Self(str)
    }
}

impl Input {
    pub fn parse<T: InputParser>(self) -> T::Output {
        T::parse_input(self.0)
    }
}