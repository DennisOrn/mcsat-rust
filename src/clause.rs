use crate::literal::Literal;
use crate::model::Model;

#[derive(Clone, Debug, PartialEq)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Clause {
        Clause { literals: literals }
    }

    pub fn get_literals(&self) -> &Vec<Literal> {
        &self.literals
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

    // TODO: move this to trail
    // #[test]
    // fn test_evaluate_clause() {
    //     let model = Model::new();

    //     assert_eq!(
    //         Clause::new(vec![Literal::new(t(), false)]).evaluate(&model),
    //         Some(true)
    //     );
    //     assert_eq!(
    //         Clause::new(vec![Literal::new(f(), false)]).evaluate(&model),
    //         Some(false)
    //     );
    //     assert_eq!(
    //         Clause::new(vec![Literal::new(t(), false), Literal::new(f(), false)]).evaluate(&model),
    //         Some(true)
    //     );
    //     assert_eq!(
    //         Clause::new(vec![
    //             Literal::new(f(), false),
    //             Literal::new(equal(variable("x"), variable("y")), false),
    //             Literal::new(f(), false),
    //         ])
    //         .evaluate(&model),
    //         None
    //     );
    // }
}
