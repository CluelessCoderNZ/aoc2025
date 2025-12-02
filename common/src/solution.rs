use std::fmt::Display;
use std::any::type_name;
use log::info;

use crate::{Input, InputParser};


pub trait ProblemQuestion: Sized {
    type Parser: InputParser;
    type Output: Display;

    fn solve<S: Solution<Self>>(input: Input) -> Self::Output {
        let solution_name = type_name::<S>();
        let problem_name = type_name::<Self>();
        info!("Running {solution_name} for {problem_name}");

        let timer = std::time::Instant::now();
        let result  = S::answer(input.parse::<Self::Parser>());
        let duration = timer.elapsed();
        
        info!("Solution Result: {result}");
        info!("Duration: {duration:?}");

        result
    }
}


pub trait Solution<P: ProblemQuestion> {
    fn answer(input: <P::Parser as InputParser>::Output) -> P::Output;
}