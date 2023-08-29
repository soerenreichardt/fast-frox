// pub type Value = f64;

use std::fmt::Display;
use miette::Result;

use crate::error::RuntimeError;

#[derive(Clone, Copy)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(boolean) => f.write_fmt(format_args!("{}", boolean)),
            Self::Number(number) => f.write_fmt(format_args!("{}", number)),
            Self::Nil => f.write_str("nil"),
        }
    }
}

impl Value {
    pub fn is_number(self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false
        }
    }

    pub fn as_number(self) -> Result<f64> {
        match self {
            Value::Number(num) => Ok(num),
            _ => Err(RuntimeError::new(format!("Cannot cast {} as number", self)).into())
        }
    }
}

impl std::ops::Neg for Value {
    type Output=Result<Value>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(number) => Ok(Value::Number(-number)),
            _ => Err(RuntimeError::new(format!("Unable to negate {}, operand must be a number", self)).into())
        }
    }
}

impl std::ops::Add for Value {
    type Output=Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_value), Value::Number(rhs_value)) => Ok(Value::Number(lhs_value + rhs_value)),
            _ => Err(RuntimeError::new(format!("Unable to add {} and {}, operands must be numbers", self, rhs)).into())
        }
    }
}

impl std::ops::Sub for Value {
    type Output=Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_value), Value::Number(rhs_value)) => Ok(Value::Number(lhs_value - rhs_value)),
            _ => Err(RuntimeError::new(format!("Unable to subtract {} and {}, operands must be numbers", self, rhs)).into())
        }
    }
}

impl std::ops::Mul for Value {
    type Output=Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_value), Value::Number(rhs_value)) => Ok(Value::Number(lhs_value * rhs_value)),
            _ => Err(RuntimeError::new(format!("Unable to multiuply {} and {}, operands must be numbers", self, rhs)).into())
        }
    }
}

impl std::ops::Div for Value {
    type Output=Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs_value), Value::Number(rhs_value)) => Ok(Value::Number(lhs_value / rhs_value)),
            _ => Err(RuntimeError::new(format!("Unable to divide {} and {}, operands must be numbers", self, rhs)).into())
        }
    }
}