use crate::formula::formula::Formula;
use hashconsing::HConsed;

#[derive(Debug)]
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

    pub fn is_negated(&self) -> bool {
        self.is_negated
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formula = self.formula.to_string();
        write!(fmt, "{}{}", if self.is_negated { "¬" } else { "" }, formula)
    }
}
