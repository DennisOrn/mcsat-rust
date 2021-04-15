#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Boolean(x) => write!(fmt, "{}", x),
            Value::Integer(x) => write!(fmt, "{}", x),
        }
    }
}
