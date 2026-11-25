use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub data: f32,
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        Value {
            data: self.data * rhs.data,
        }
    }
}

impl Mul for &Value {
    type Output = Value;

    fn mul(self, rhs: &Value) -> Value {
        Value {
            data: self.data * rhs.data,
        }
    }
}

/// provides += functionality
impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Value) {
        self.data += rhs.data;
    }
}

/// provides summation
impl Sum for Value {
    fn sum<I: Iterator<Item = Value>>(iter: I) -> Value {
        iter.fold(Value { data: 0.0 }, |acc, v| acc + v)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        Value {
            data: self.data + rhs.data,
        }
    }
}

impl Add for &Value {
    type Output = Value;

    fn add(self, rhs: &Value) -> Value {
        Value {
            data: self.data + rhs.data,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value(data={})", self.data)
    }
}

impl Value {
    pub fn new(data: f32) -> Self {
        Value { data }
    }
}
