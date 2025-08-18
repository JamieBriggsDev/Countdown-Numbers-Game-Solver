// ops.rs
use std::fmt;

pub struct Add;
pub struct Subtract;
pub struct BitShiftLeft;
pub struct BitShiftRight;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    NegativeShift,
    ShiftOutOfRange,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::NegativeShift => write!(f, "shift count must be non-negative"),
            CalcError::ShiftOutOfRange => write!(f, "shift count out of range for i64"),
        }
    }
}

pub trait Calculation {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError>;
    fn symbol(&self) -> &str;
}

impl Calculation for Add {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        Ok(x + y)
    }
    fn symbol(&self) -> &str {
        "+"
    }
}

impl Calculation for Subtract {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        Ok(x - y)
    }

    fn symbol(&self) -> &str {
        "-"
    }
}

impl Calculation for BitShiftLeft {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        let shift: u32 = y.try_into().map_err(|_| CalcError::NegativeShift)?;
        x.checked_shl(shift).ok_or(CalcError::ShiftOutOfRange)
    }

    fn symbol(&self) -> &str {
        "<<"
    }
}

impl Calculation for BitShiftRight {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        let shift: u32 = y.try_into().map_err(|_| CalcError::NegativeShift)?;
        x.checked_shr(shift).ok_or(CalcError::ShiftOutOfRange)
    }

    fn symbol(&self) -> &str {
        ">>"
    }
}

pub fn default_operations() -> Vec<Box<dyn Calculation>> {
    vec![
        Box::new(Add),
        Box::new(Subtract),
        Box::new(BitShiftLeft),
        Box::new(BitShiftRight),
    ]
}