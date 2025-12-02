use std::fmt::Display;
use std::any::type_name;
use crate::{Input, InputParser};


pub trait ProblemQuestion: Sized {
    type Parser: InputParser;
    type Output: Display;

    fn solve<S: Solution<Self>>(input: Input) -> Self::Output {
        let solution_name = type_name::<S>();
        let problem_name = type_name::<Self>();
        println!("Running Solution {solution_name} for {problem_name}\n\n");

        let result  = S::answer(input.parse::<Self::Parser>());
        println!("Result: {result}\n\n");

        result
    }
}


pub trait Solution<P: ProblemQuestion> {
    fn answer(input: <P::Parser as InputParser>::Output) -> P::Output;
}