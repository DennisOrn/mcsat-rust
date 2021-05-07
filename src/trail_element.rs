use crate::clause::Clause;
use crate::literal::Literal;
use crate::types::value::Value;
use crate::types::variable::Variable;

#[derive(Clone)]
pub enum TrailElement {
    DecidedLiteral(Literal),
    PropagatedLiteral(Clause, Literal),
    ModelAssignment(Variable, Value), // TODO: should Variable be HConsed?
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
