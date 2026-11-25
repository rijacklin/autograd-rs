use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub data: f32,
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
