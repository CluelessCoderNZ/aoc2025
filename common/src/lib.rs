mod input;
pub use input::Input;

mod parser;
pub use parser::InputParser;
pub use parser::LineInputParser;

mod solution;
pub use solution::ProblemQuestion;
pub use solution::Solution;


pub fn init_logger() {
    env_logger::builder()
    .parse_env(env_logger::Env::default().default_filter_or("info"))
    .target(env_logger::Target::Stdout)
    .init();
}