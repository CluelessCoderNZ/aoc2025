use std::collections::HashSet;

use common::{Input, ProblemQuestion, Solution};
use itertools::Itertools;
use crate::parser::{Point3D, Point3DParser};

mod parser;

pub const TEST_EXAMPLE: Input = Input::from_str(include_str!("../example"));
pub const TEST_INPUT: Input = Input::from_str(include_str!("../input"));

struct ProblemOne;
impl ProblemQuestion for ProblemOne {
    type Parser = Point3DParser;
    type Output = u64;
}

// struct ProblemTwo;
// impl ProblemQuestion for ProblemTwo {
//     type Parser = Point3DParser;
//     type Output = u64;
// }

struct DayEightSolution<const N: usize>;

impl<const N: usize> DayEightSolution<N> {

    fn get_shortest_pairs(input: &Vec<Point3D>) -> impl Iterator<Item = (Point3D, Point3D)> {
        input.iter()
            .tuple_combinations::<(&Point3D, &Point3D)>()
            .map(|(a, b)| (a.clone(), b.clone()))
            .sorted_unstable_by_key(|(a,b)| (*a - *b).square_length())
            .take(N)
    }

    fn form_circuits(input: &Vec<Point3D>) -> Vec<HashSet<Point3D>> {
        let mut circuits: Vec<HashSet<Point3D>> = Vec::new();
        
        for (a, b) in Self::get_shortest_pairs(&input) {
            let circuit_a_maybe = circuits.iter().position(|set: &HashSet<Point3D>| set.contains(&a));
            let circuit_b_maybe = circuits.iter().position(|set: &HashSet<Point3D>| set.contains(&b));

            match (circuit_a_maybe, circuit_b_maybe) {
                (Some(index_a), Some(index_b)) => {
                    if index_a != index_b {
                        let circuit_b = circuits[index_b].clone();
                        circuits[index_a].extend(circuit_b);
                        circuits.remove(index_b);
                    }
                },
                (None, Some(index_b)) => {
                    circuits[index_b].insert(a);
                },
                (Some(index_a), None) => {
                    circuits[index_a].insert(b);
                },
                (None, None) => {
                    circuits.push(HashSet::from([a, b]));
                }
            }
        }

        circuits
    }
}

impl<const N: usize> Solution<ProblemOne> for DayEightSolution<N> {
    fn answer(input: Vec<Point3D>) -> u64 {
        let mut circuits = Self::form_circuits(&input);
        circuits.sort_by_key(|circuit| circuit.len());

        for circuit in circuits.iter().rev()
            .map(|circuit| circuit.len() as u64)
            .take(3) {
                println!("{circuit}")
            }
        
        circuits.iter().rev()
            .map(|circuit| circuit.len() as u64)
            .take(3)
            .product::<u64>()
    }
}

fn main() {
    // 381840 Too High
    ProblemOne::solve::<DayEightSolution<1000>>(TEST_INPUT);
}

#[cfg(test)]
mod test {
    use common::ProblemQuestion;

    use crate::{DayEightSolution, ProblemOne, TEST_EXAMPLE};


    #[test]
    fn test_problem_one_example() {
        let result = ProblemOne::solve::<DayEightSolution<10>>(TEST_EXAMPLE);
        assert_eq!(result, 40)
    }
}