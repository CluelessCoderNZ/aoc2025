use std::{collections::VecDeque, fmt::Display};

pub type IndicatorState = u16;

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
