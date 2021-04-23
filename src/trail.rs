use crate::clause::Clause;
use crate::literal::Literal;
use crate::model::Model;
use crate::types::value::Value;
use crate::types::variable::Variable;

#[derive(Debug)]
pub struct Trail {
    elements: Vec<TrailElement>,
    model: Model,
}

impl Trail {
    pub fn new() -> Trail {
        Trail {
            elements: Vec::new(),
            model: Model::new(),
        }
    }

    pub fn push_decided_literal(&mut self, literal: Literal) {
        self.elements.push(TrailElement::DecidedLiteral(literal))
    }

    pub fn push_propagated_literal(&mut self, clause: Clause, literal: Literal) {
        self.elements
            .push(TrailElement::PropagatedLiteral(clause, literal))
    }

    pub fn push_model_assignment(&mut self, variable: Variable, value: Value) {
        self.elements
            .push(TrailElement::ModelAssignment(variable.clone(), value));
        self.model.set_value(variable, value);
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        self.elements.pop()
        // TODO: remove from model if model assignment
    }

    pub fn value(&self, literal: &Literal) -> Option<bool> {
        let value_b = self.value_b(literal);
        if value_b.is_none() {
            return self.value_t(literal);
        } else {
            return value_b;
        }
    }

    pub fn value_b(&self, literal: &Literal) -> Option<bool> {
        let literals = self.all_literals();
        let negated_literal = literal.negate();
        for l in literals {
            if l == literal {
                return Some(true);
            } else if l == &negated_literal {
                return Some(false);
            }
        }
        None
    }

    pub fn value_t(&self, literal: &Literal) -> Option<bool> {
        // let model_assignments = self.all_assignments();
        // let mut model_clone = self.model.clone();

        // for (var, val) in model_assignments {
        //     println!("updating model: {} = {}", var, val);
        //     model_clone.set_value(var.clone(), val);
        //     match literal.evaluate(&model_clone) {
        //         Some(true) => {
        //             println!("evaluation returned true, return true\n");
        //             return Some(true);
        //         }
        //         Some(false) => {
        //             println!("evaluation returned false, return false\n");
        //             return Some(false);
        //         }
        //         _ => (),
        //     }
        //     model_clone.clear_value(var.clone());
        // }

        // println!("undefined, return None\n");
        // None

        literal.evaluate(&self.model)
    }

    pub fn is_consistent(&self) -> bool {
        for l in self.all_literals() {
            match self.value_t(l) {
                Some(false) => return false,
                _ => (),
            }
        }
        true
    }

    pub fn is_complete(&self) -> bool {
        for l in self.all_literals() {
            match self.value_t(l) {
                Some(false) | None => return false,
                _ => (),
            }
        }
        true
    }

    fn all_literals(&self) -> Vec<&Literal> {
        // TODO: inefficient to loop each time function is called.
        self.elements
            .iter()
            .flat_map(|x| match x {
                TrailElement::DecidedLiteral(l) => Some(l),
                TrailElement::PropagatedLiteral(_, l) => Some(l),
                _ => None,
            })
            .collect()
    }

    // fn all_assignments(&self) -> Vec<(Variable, Value)> {
    //     // TODO: inefficient to loop each time function is called.
    //     self.elements
    //         .iter()
    //         .flat_map(|x| match x {
    //             TrailElement::ModelAssignment(var, val) => Some((var.clone(), *val)),
    //             _ => None,
    //         })
    //         .collect()
    // }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::formula::formula::greater;
    use crate::literal::Literal;
    use crate::term::term::{constant, variable};
    use crate::trail::Trail;
    use crate::types::value::Value;
    use crate::types::variable::Variable;

    #[test]
    fn test_trail_value_functions() {
        // EXAMPLE 1 FROM MCSAT-PAPER
        // M = [x > 1, x ↦ 1, y ↦ 0, z > 0]
        let mut trail = Trail::new();
        trail.push_decided_literal(Literal::new(
            greater(variable("x"), constant(Value::Integer(0))),
            false,
        ));
        trail.push_model_assignment(Variable::new("x"), Value::Integer(1));
        trail.push_model_assignment(Variable::new("y"), Value::Integer(0));
        trail.push_decided_literal(Literal::new(
            greater(variable("z"), constant(Value::Integer(0))),
            false,
        ));

        assert_eq!(
            trail.value_t(&Literal::new(
                greater(variable("x"), constant(Value::Integer(0))),
                false
            )),
            Some(true),
            "expected: value_t(x > 0) == true"
        );
        assert_eq!(
            trail.value_b(&Literal::new(
                greater(variable("x"), constant(Value::Integer(0))),
                false
            )),
            Some(true),
            "expected: value_b(x > 0) == true"
        );
        assert_eq!(
            trail.value_t(&Literal::new(
                greater(variable("x"), constant(Value::Integer(1))),
                false
            )),
            Some(false),
            "expected: value_t(x > 1) == false"
        );
        assert_eq!(
            trail.value_t(&Literal::new(
                greater(variable("z"), constant(Value::Integer(0))),
                false
            )),
            None,
            "expected: value_t(z > 0) == None"
        );
        assert_eq!(
            trail.value_b(&Literal::new(
                greater(variable("z"), constant(Value::Integer(0))),
                false
            )),
            Some(true),
            "expected: value_b(z > 0) == true"
        );
    }
}
