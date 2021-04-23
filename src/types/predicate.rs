use crate::model::Model;
use crate::types::value::Value;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Predicate {
    // TODO: implement hashconsing for this?
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
}
impl Predicate {
    pub fn evaluate(&self, model: &Model, args: &Vec<Value>) -> bool {
        assert!(args.len() == 2);
        match self {
            Predicate::Less => args[0] < args[1],
            Predicate::LessEqual => args[0] <= args[1],
            Predicate::Greater => args[0] > args[1],
            Predicate::GreaterEqual => args[0] >= args[1],
            Predicate::Equal => args[0] == args[1],
        }
    }
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Predicate::Less => write!(fmt, "<"),
            Predicate::Greater => write!(fmt, ">"),
            Predicate::LessEqual => write!(fmt, "≤"),
            Predicate::GreaterEqual => write!(fmt, "≥"),
            Predicate::Equal => write!(fmt, "="),
        }
    }
}
