use std::{collections::VecDeque, fmt::Display};
use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};

pub type IndicatorState = u16;

#[derive(Clone)]
pub struct Button {
    pub toggle_pattern: Vec<usize>,
}

impl Button {
    pub fn as_bitmask(&self) -> IndicatorState {
        let mut mask = 0;

        for index in &self.toggle_pattern {
            mask |= 1 << index;
        }

        mask
    }

    fn add_joltage(&self, joltage: &mut Vec<u64>, presses: u64) {
        for index in &self.toggle_pattern {
            joltage[*index] += presses;
        }
    }
}

pub struct Machine {
    pub indicator_pattern: Vec<bool>,
    pub button_wiring: Vec<Button>,
    pub joltage_req: Vec<u64>,
}

impl Machine {
    pub fn indicator_bitmask(&self) -> IndicatorState {
        let mut mask = 0;

        for (index, state) in self.indicator_pattern.iter().enumerate() {
            if *state {
                mask |= 1 << index;
            }
        }

        mask
    }

    pub fn min_startup_presses(&self) -> u64 {
        let goal: IndicatorState = self.indicator_bitmask();
        let buttons: Vec<IndicatorState> = self
            .button_wiring
            .iter()
            .map(|button| button.as_bitmask())
            .collect();

        Self::count_state_jumps(0, goal, &buttons)
    }

    fn count_state_jumps(
        inital: IndicatorState,
        goal: IndicatorState,
        buttons: &Vec<IndicatorState>,
    ) -> u64 {
        let mut state_queue: VecDeque<(IndicatorState, u64)> = VecDeque::from([(inital, 0)]);

        while let Some((state, level)) = state_queue.pop_front() {
            if state == goal {
                return level;
            }

            for button in buttons {
                state_queue.push_back((state ^ button, level + 1));
            }
        }

        unreachable!()
    }
}

impl Machine {
    pub fn min_joltage_presses(&self) -> u64 {
        let solution = self.get_min_joltage_solution();
        self.test_joltage_solution(&solution).expect("Solution is valid");
        
        solution.iter().sum()
    }

    fn test_joltage_solution(&self, solution: &Vec<u64>) -> Result<(), Vec<u64>> {
        let mut joltages = vec![0u64; self.joltage_req.len()];

        // Add button presses to joltage
        for (index, button_presses) in solution.iter().enumerate() {
            self.button_wiring[index].add_joltage(&mut joltages, *button_presses);
        }

        // Check if meets requirements
        if self.joltage_req == joltages {
            return Ok(());
        } else {
            return Err(joltages);
        }
    }

    /// Returns presses for each button in order to get min solution
    fn get_min_joltage_solution(&self) -> Vec<u64> {
        let (problem, vars) = self.form_sys_equations_problem();
        let solution = problem.solve().expect("All machines solveable");
        
        vars.into_iter()
            .map(|(_, var)| solution.var_value_rounded(var) as u64)
            .collect()
    }

    fn form_sys_equations_problem(&self) -> (Problem, Vec<(Button, Variable)>) {
        // Minimise system of equations where each button press is an independent variable
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let button_vars: Vec<_> = self.button_wiring.iter()
            .map(|button| (button.clone(), problem.add_integer_var(1.0, (0, i32::MAX))))
            .collect();

        // Constraints formed from sum of wired button presses for each joltage req
        for (index, joltage) in self.joltage_req.iter().enumerate() {
            let wired_variables = button_vars.iter()
                .filter(|(button, _)| button.toggle_pattern.contains(&index))
                .map(|(_, var)| (var.clone(), 1.0));
            
            problem.add_constraint(wired_variables, ComparisonOp::Eq, *joltage as f64);
        }

        (problem, button_vars)
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = self
            .toggle_pattern
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "({})", value_str)
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indicator_str = self
            .indicator_pattern
            .iter()
            .map(|val| {
                if *val {
                    String::from("#")
                } else {
                    String::from(".")
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let button_str = self
            .button_wiring
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let joltage_str = self
            .joltage_req
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "[{}] {} {{{}}}", indicator_str, button_str, joltage_str)
    }
}
