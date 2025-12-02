#[derive(Debug)]
pub enum DialInstruction {
    Left(i32),
    Right(i32)
}

#[derive(Debug)]
pub struct DialPosition {
    pos: i32
}

impl Default for DialPosition {
    fn default() -> Self {
        Self {
            pos: 50
        }
    }
}

impl DialPosition {
    const MAX_POSITION: i32 = 100;

    /// Move dial by amount, returns number of times dial passes through zero
    fn move_by(&mut self, amount: i32) -> i32 {
        let mut click_count = amount.abs() / Self::MAX_POSITION;
        let amount = amount % Self::MAX_POSITION;

        let nowrap_pos = self.pos + amount;
        let wrap_pos = nowrap_pos.rem_euclid(Self::MAX_POSITION);

        let starts_on_zero = self.pos == 0;
        let ends_on_zero = wrap_pos == 0 && amount != 0;
        let moves_through_zero = !starts_on_zero && nowrap_pos != wrap_pos;

        if ends_on_zero || moves_through_zero {
            click_count += 1;
        }

        self.pos = wrap_pos;
        return click_count;
    }

    /// Applies a dial instruction returns the number of times the dial passes through zero
    pub fn apply(&mut self, action: DialInstruction) -> i32 {
        let click_count = match action {
            DialInstruction::Left(val) => self.move_by(-val),
            DialInstruction::Right(val) => self.move_by(val),
        };

        let pos = self.pos();
        println!("Move {action:?} -> {pos} ({click_count})");
        
        click_count
    }

    pub fn pos(&self) -> i32 {
        return self.pos;
    }
}

pub fn count_zeros(instructions: Vec<DialInstruction>) -> usize {
    let mut dial = DialPosition::default();

    instructions.into_iter().map(|instr| {
        dial.apply(instr);
        dial.pos()
    }).filter(|pos| *pos == 0)
    .count()
}

pub fn count_clicks(instructions: Vec<DialInstruction>) -> i32 {
    let mut dial = DialPosition::default();

    instructions.into_iter()
    .map(|instr| dial.apply(instr))
    .sum()
}


#[cfg(test)]
mod test {
    use super::{
        DialInstruction,
        count_clicks
    };

    #[test]
    fn test_rollover() {
        let instructions = vec![
            DialInstruction::Left(1000)
        ];
        assert_eq!(count_clicks(instructions), 10)
    }

    #[test]
    fn test_rollover_remainder() {
        let instructions = vec![
            DialInstruction::Left(1050)
        ];
        assert_eq!(count_clicks(instructions), 11)
    }

    #[test]
    fn test_rollover_remainder_rollover() {
        let instructions = vec![
            DialInstruction::Left(1051)
        ];
        assert_eq!(count_clicks(instructions), 11)
    }

}