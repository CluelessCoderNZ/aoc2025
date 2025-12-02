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
        info!("Running Solution {solution_name} for {problem_name}");

        let result  = S::answer(input.parse::<Self::Parser>());
        info!("Solution Result: {result}");

        result
    }
}


pub trait Solution<P: ProblemQuestion> {
    fn answer(input: <P::Parser as InputParser>::Output) -> P::Output;
}