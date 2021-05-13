use crate::clause::Clause;
use crate::literal::Literal;
use crate::term::term::Term;
use crate::types::value::Value;
use hashconsing::HConsed;

#[derive(Clone)]
pub enum TrailElement {
    DecidedLiteral(Literal),
    PropagatedLiteral(Clause, Literal),
    ModelAssignment(HConsed<Term>, Value),
}

impl std::fmt::Display for TrailElement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TrailElement::DecidedLiteral(f) => write!(fmt, "{}", f),
            TrailElement::PropagatedLiteral(c, f) => write!(fmt, "{} → {}", c, f),
            TrailElement::ModelAssignment(var, val) => write!(fmt, "{} ↦ {}", var, val),
        }
    }
}
