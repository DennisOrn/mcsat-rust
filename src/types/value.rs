use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i32),
    True,
    False,
}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(lhs + rhs),
            _ => panic!("Cannot add {} and {}", self, other),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(lhs - rhs),
            _ => panic!("Cannot subtract {} and {}", self, other),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(x) => write!(fmt, "{}", x),
            Value::True => write!(fmt, "True"),
            Value::False => write!(fmt, "False"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::value::Value;

    #[test]
    fn test_addition_and_subtraction() {
        assert_eq!(Value::Integer(1) + Value::Integer(2), Value::Integer(3));
        assert_eq!(Value::Integer(2) - Value::Integer(1), Value::Integer(1));
        assert_eq!(Value::Integer(-1) - Value::Integer(-1), Value::Integer(0));
    }

    #[test]
    #[should_panic]
    fn test_bad_addition() {
        let _ = Value::Integer(1) + Value::True;
    }

    #[test]
    #[should_panic]
    fn test_bad_subtraction() {
        let _ = Value::Integer(1) - Value::True;
    }
}
