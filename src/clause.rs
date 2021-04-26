use crate::literal::Literal;
use crate::model::Model;

#[derive(Debug)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Clause {
        Clause { literals: literals }
    }

    pub fn evaluate(&self, model: &Model) -> Option<bool> {
        let mut found_undefined_literal = false;
        for literal in &self.literals {
            match literal.evaluate(model) {
                Some(true) => return Some(true),
                None => found_undefined_literal = true,
                _ => (),
            }
        }

        if found_undefined_literal {
            None
        } else {
            Some(false)
        }
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formulas_list: Vec<String> = self.literals.iter().map(|x| x.to_string()).collect();
        write!(fmt, "{}", formulas_list.join(" ∨ "))
    }
}

#[cfg(test)]
mod tests {
    use crate::clause::Clause;
    use crate::formula::formula::equal;
    use crate::formula::formula::{f, t};
    use crate::literal::Literal;
    use crate::model::Model;
    use crate::term::term::variable;

    #[test]
    fn test_evaluate_clause() {
        let model = Model::new();

        assert_eq!(
            Clause::new(vec![Literal::new(t(), false)]).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            Clause::new(vec![Literal::new(f(), false)]).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            Clause::new(vec![Literal::new(t(), false), Literal::new(f(), false)]).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            Clause::new(vec![
                Literal::new(f(), false),
                Literal::new(equal(variable("x"), variable("y")), false),
                Literal::new(f(), false),
            ])
            .evaluate(&model),
            None
        );
    }
}
