use std::fmt::Display;
use std::any::type_name;
use log::info;

use crate::{
    Input, 
    InputParser
};

fn run_timed<F, T>(name: &str, method: F) -> T
    where F: FnOnce() -> T 
{
    let timer = std::time::Instant::now();
    let result = method();
    let duration = timer.elapsed();

    log::info!("{name} {duration:?}");

    return result;
}

pub trait ProblemQuestion: Sized {
    type Parser: InputParser;
    type Output: Display;

    fn solve<S: Solution<Self>>(input: Input) -> Self::Output {
        let solution_name = type_name::<S>();
        let problem_name = type_name::<Self>();
        info!("Solving {problem_name} with {solution_name}");

        let result = run_timed(
            "Total:             ", 
            || {
                let parsed_input = run_timed(
                    "Parse Input:       ", 
                    || input.parse::<Self::Parser>()
                );

                run_timed(
                    "Calculating Answer:", 
                    || S::answer(parsed_input)
                )
            }
        );

        info!("Solution Result: {result}\n");
        result
    }
}


pub trait Solution<P: ProblemQuestion> {
    fn answer(input: <P::Parser as InputParser>::Output) -> P::Output;
}