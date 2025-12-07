use std::{fmt::Display, ops::{Add, Mul}, str::FromStr};

use common::Grid2D;


#[derive(Debug, Clone, Copy)]
pub enum MathOperator {
    Add,
    Multiply
}

impl MathOperator {
    pub fn apply<T>(&self, a: T, b: T) -> T where 
        T: Add<Output = T>,
        T: Mul<Output = T>
    {
        match self {
            Self::Add => a.add(b),
            Self::Multiply => a.mul(b)
        }
    }

    pub fn identity(&self) -> u64 {
        match self {
            MathOperator::Add => 0,
            MathOperator::Multiply => 1,
        }
    } 
}

impl Display for MathOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathOperator::Add => write!(f, "+"),
            MathOperator::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MathCell {
    Number(u64),
    Op(MathOperator)
}

impl FromStr for MathCell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('+') {
            return Ok(Self::Op(MathOperator::Add));
        } else if s.contains('*') {
            return Ok(Self::Op(MathOperator::Multiply));
        } else if let Ok(val) = u64::from_str(s) {
            return Ok(Self::Number(val));
        }

        return Err(());
    }
}

impl MathCell {
    pub fn as_op(self) -> Option<MathOperator> {
        match self {
            Self::Op(op) => Some(op),
            _ => None
        }
    }

    pub fn as_num(self) -> Option<u64> {
        match self {
            Self::Number(val) => Some(val),
            _ => None
        }
    }
}

impl Display for MathCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathCell::Number(val) => write!(f, "{:^5}", val),
            MathCell::Op(math_operator) => write!(f, "{:^5}", math_operator.to_string()),
        }
    }
}

pub type MathHomework = Grid2D<MathCell>;

pub fn get_equations(homework: &MathHomework) -> impl Iterator<Item = (Vec<u64>, MathOperator)> {
    homework.columns_iter().map(|column| {
        let mut column: Vec<MathCell> = column.map(|point| homework.get(point).unwrap().clone()).collect();
        let op = column.pop()
            .map(|cell| cell.as_op().expect("Last item is operator"))
            .expect("At least one row of operators");
        let values = column.into_iter().map(|cell| cell.as_num().expect("Column composed of values")).collect();

        (values, op)
    })
}