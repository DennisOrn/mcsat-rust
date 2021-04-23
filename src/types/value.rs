use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i32),
}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(lhs + rhs),
            _ => panic!(
                "Could not perform addition on values {} and {}",
                self, other
            ),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(lhs - rhs),
            _ => panic!(
                "Could not perform subtraction on values {} and {}",
                self, other
            ),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(x) => write!(fmt, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::term::term::{constant, minus, plus};
    use crate::types::value::Value;

    #[test]
    fn test_value_addition_and_subtraction() {
        assert_eq!(Value::Integer(1) + Value::Integer(2), Value::Integer(3));
        assert_eq!(Value::Integer(2) - Value::Integer(1), Value::Integer(1));
    }
}
