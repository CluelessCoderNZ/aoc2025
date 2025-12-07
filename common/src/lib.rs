mod input;
pub use input::Input;

mod parser;
pub use parser::InputParser;
pub use parser::LineInputParser;
pub use parser::DualSectionParser;
pub use parser::InclusiveRangeListParser;
pub use parser::FromStrParser;

mod solution;
pub use solution::ProblemQuestion;
pub use solution::Solution;

mod grid;
pub use grid::{
    GridUnit,
    Point2D,
    Vector2D,
    Size2D,
    Rect,
};
pub use grid::{
    Direction,
    CardinalDirection,
    OrdinalDirection,
};
pub use grid::{
    Grid2D,
    Grid2DParser,
    Grid2DWhitespaceParser
};

fn init_logger(is_test: bool) {
    env_logger::builder()
    .parse_env(env_logger::Env::default().default_filter_or("info"))
    .target(env_logger::Target::Stdout)
    .is_test(is_test)
    .init();
}

#[cfg(not(test))]
#[ctor::ctor]
fn init() {
    init_logger(false);
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    init_logger(true);
}