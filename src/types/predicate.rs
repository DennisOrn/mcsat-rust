use crate::model::Model;
use crate::term::term::Term;
use hashconsing::HConsed;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Predicate {
    // TODO: implement hashconsing for this?
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
}
impl Predicate {
    pub fn evaluate(&self, model: &Model, args: &Vec<HConsed<Term>>) -> Option<bool> {
        println!("eval predicate\t{}", self);
        match self {
            // Predicate::LessThan => Some(args[0].evaluate(model) < args[1].evaluate(model)),
            // Predicate::LessThanOrEqual => Some(args[0].evaluate(model) <= args[1].evaluate(model)),
            // Predicate::GreaterThan => Some(args[0].evaluate(model) > args[1].evaluate(model)),
            // Predicate::GreaterThanOrEqual => Some(args[0].evaluate(model) >= args[1].evaluate(model)),
            Predicate::Equal => {
                let lhs = args[0].evaluate(model);
                let rhs = args[1].evaluate(model); // TODO: do lazy evaluation.
                match (lhs, rhs) {
                    (None, _) | (_, None) => None,
                    _ => Some(lhs == rhs),
                }
            }

            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Predicate::LessThan => write!(fmt, "<"),
            Predicate::GreaterThan => write!(fmt, ">"),
            Predicate::LessThanOrEqual => write!(fmt, "≤"),
            Predicate::GreaterThanOrEqual => write!(fmt, "≥"),
            Predicate::Equal => write!(fmt, "="),
        }
    }
}
