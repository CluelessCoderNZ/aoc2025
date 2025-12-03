use std::fmt::Display;

pub struct BatteryBank {
    pub batteries: Vec<u8>
}

impl Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.batteries.iter()
            .map(|v| char::from_digit(*v as u32, 10).expect("All values are below 10"))
            .collect::<String>();

        write!(f, "{}", str)
    }
}

impl BatteryBank {
    pub fn max_joltage_indices(&self, n: usize) -> Vec<usize> {
        let mut output = Vec::with_capacity(n);
        let mut search_start = 0;

        for i in 0..n {
            let search_end = self.batteries.len()-n+i+1;

            let next =self.batteries[search_start..search_end]
                .iter().enumerate().rev() // Reversed so to grab first amongst equal
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| search_start + index)
                .expect("bank has at least n elements");

            output.push(next);
            search_start = next + 1;
        }

        output
    }

    fn log_indicies(&self, indices: &Vec<usize>) {
        let highlight_str: String = (0..self.batteries.len())
            .map(|i| if indices.contains(&i) {'*'} else {' '})
            .collect();

        log::debug!("\n{}\n{}", highlight_str, self);
    }

    pub fn max_joltage(&self, n: usize) -> u64 {
        let indices = self.max_joltage_indices(n);
        self.log_indicies(&indices);
        indices.into_iter()
            .rev()
            .enumerate()
            .map(|(i, battery_index)| 10u64.pow(i as u32) * self.batteries[battery_index] as u64)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use common::LineInputParser;

    use crate::parser::BatteryBankParser;


    #[test]
    fn test_example1_joltage() {
        let bank = BatteryBankParser::parse_line("987654321111111");
        assert_eq!(bank.max_joltage(2), 98);
    }

    #[test]
    fn test_example2_joltage() {
        let bank = BatteryBankParser::parse_line("811111111111119");
        assert_eq!(bank.max_joltage(2), 89);
    }

    #[test]
    fn test_example3_joltage() {
        let bank = BatteryBankParser::parse_line("234234234234278");
        assert_eq!(bank.max_joltage(2), 78);
    }

    #[test]
    fn test_example4_joltage() {
        let bank = BatteryBankParser::parse_line("818181911112111");
        assert_eq!(bank.max_joltage(2), 92);
    }

    #[test]
    fn test_equal_first_break() {
        let bank = BatteryBankParser::parse_line("999991");
        assert_eq!(bank.max_joltage(2), 99);
    }
}