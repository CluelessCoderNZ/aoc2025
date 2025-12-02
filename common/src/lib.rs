mod input;
pub use input::Input;

mod parser;
pub use parser::InputParser;
pub use parser::LineInputParser;

mod solution;
pub use solution::ProblemQuestion;
pub use solution::Solution;


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