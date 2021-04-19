use crate::model::Model;
use crate::term::term::Term;
use crate::types::value::Value;
use hashconsing::HConsed;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Function {
    // TODO: implement hashconsing for this?
    Plus,
    Minus,
}
impl Function {
    pub fn evaluate(&self, model: &Model, args: &Vec<HConsed<Term>>) -> Option<Value> {
        match self {
            // TODO: do lazy evaluation.
            Function::Plus => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs + rhs),
            },
            Function::Minus => match (args[0].evaluate(model), args[1].evaluate(model)) {
                (None, _) | (_, None) => None,
                (Some(lhs), Some(rhs)) => Some(lhs - rhs),
            },
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Function::Plus => write!(fmt, "+"),
            Function::Minus => write!(fmt, "-"),
        }
    }
}
