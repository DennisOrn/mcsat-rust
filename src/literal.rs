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

#[cfg(test)]
mod tests {
    use crate::formula::formula::equal;
    use crate::formula::formula::{f, t};
    use crate::literal::Literal;
    use crate::model::Model;
    use crate::term::term::variable;

    #[test]
    fn test_evaluate_literal() {
        let model = Model::new();

        assert_eq!(Literal::new(t(), false).evaluate(&model), Some(true));
        assert_eq!(Literal::new(t(), true).evaluate(&model), Some(false));
        assert_eq!(Literal::new(f(), false).evaluate(&model), Some(false));
        assert_eq!(Literal::new(f(), true).evaluate(&model), Some(true));
        assert_eq!(
            Literal::new(equal(variable("x"), variable("y")), false).evaluate(&model),
            None
        );
        assert_eq!(
            Literal::new(equal(variable("x"), variable("y")), true).evaluate(&model),
            None
        );
    }
}
