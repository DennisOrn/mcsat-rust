use crate::formula::formula::Formula;
use crate::model::Model;
use hashconsing::HConsed;

#[derive(Debug, PartialEq)]
pub struct Literal {
    // TODO: should literals be hashconsed?
    formula: HConsed<Formula>,
    is_negated: bool,
}

impl Literal {
    pub fn new(formula: HConsed<Formula>, is_negated: bool) -> Literal {
        Literal {
            formula: formula,
            is_negated: is_negated,
        }
    }

    pub fn negate(&self) -> Literal {
        Literal {
            formula: self.formula.clone(),
            is_negated: !self.is_negated,
        }
    }

    pub fn evaluate(&self, model: &Model) -> Option<bool> {
        self.formula
            .evaluate(model)
            .map(|b| if self.is_negated { !b } else { b })
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formula = self.formula.to_string();
        write!(fmt, "{}{}", if self.is_negated { "¬" } else { "" }, formula)
    }
}
