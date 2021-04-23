use crate::model::Model;
use crate::types::value::Value;
use hashconsing::HConsed;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Function {
    // TODO: implement hashconsing for this?
    Plus,
    Minus,
}
impl Function {
    pub fn evaluate(&self, model: &Model, args: &Vec<Value>) -> Value {
        assert!(args.len() == 2);
        match self {
            Function::Plus => args[0] + args[1],
            Function::Minus => args[0] - args[1],
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
