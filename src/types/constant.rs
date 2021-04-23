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
