use crate::formula::formula::Formula;
use crate::model::Model;
use crate::types::variable::Variable;
use hashconsing::HConsed;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Literal {
    // TODO: should literals be hashconsed?
    // TODO: Check all data types, hashconsing.
    formula: HConsed<Formula>,
    is_negated: bool,
    variables: Vec<Variable>,
}

impl Literal {
    pub fn new(formula: HConsed<Formula>, is_negated: bool, variables: Vec<Variable>) -> Literal {
        Literal {
            formula: formula,
            is_negated: is_negated,
            variables: variables,
        }
    }

    pub fn negate(&self) -> Literal {
        Literal {
            formula: self.formula.clone(),
            is_negated: !self.is_negated,
            variables: self.variables.clone(),
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
    use crate::types::variable::Variable;

    #[test]
    fn test_evaluate_literal() {
        let model = Model::new();

        assert_eq!(
            Literal::new(t(), false, vec![]).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            Literal::new(t(), true, vec![]).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            Literal::new(f(), false, vec![]).evaluate(&model),
            Some(false)
        );
        assert_eq!(Literal::new(f(), true, vec![]).evaluate(&model), Some(true));
        assert_eq!(
            Literal::new(
                equal(variable("x"), variable("y")),
                false,
                vec![Variable::new("x"), Variable::new("y")]
            )
            .evaluate(&model),
            None
        );
        assert_eq!(
            Literal::new(
                equal(variable("x"), variable("y")),
                true,
                vec![Variable::new("x"), Variable::new("y")]
            )
            .evaluate(&model),
            None
        );
    }
}
