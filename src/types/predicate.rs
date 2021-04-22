use crate::model::Model;
use crate::term::term::Term;
use hashconsing::HConsed;

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
    pub fn evaluate(&self, model: &Model, args: &Vec<HConsed<Term>>) -> Option<bool> {
        match self {
            // TODO: do lazy evaluation.
            Predicate::Less => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs < rhs),
            },
            Predicate::LessEqual => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs <= rhs),
            },
            Predicate::Greater => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs > rhs),
            },
            Predicate::GreaterEqual => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs >= rhs),
            },
            Predicate::Equal => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs == rhs),
            },
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
