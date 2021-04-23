use crate::types::value::Value;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Constant {
    value: Value,
}
impl Constant {
    pub fn new(value: Value) -> Constant {
        Constant { value: value }
    }
    pub fn evaluate(&self) -> Value {
        self.value
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::constant::Constant;
    use crate::types::value::Value;

    #[test]
    fn test_evaluate_constant() {
        assert_eq!(
            Constant::new(Value::Integer(5)).evaluate(),
            Value::Integer(5)
        );
    }
}
