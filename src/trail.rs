use crate::clause::Clause;
use crate::literal::Literal;
use crate::model::Model;
use crate::term::term::Term;
use crate::trail_element::TrailElement;
use crate::types::value::Value;
use crate::types::variable::Variable;
use hashconsing::HConsed;

#[derive(Clone)]
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

    pub fn push_decided_literal(&mut self, literal: &Literal) {
        let element = TrailElement::DecidedLiteral(literal.clone());
        // println!("Push decided literal: {}", element);
        self.elements.push(element)
    }

    pub fn push_propagated_literal(&mut self, clause: &Clause, literal: &Literal) {
        let element = TrailElement::PropagatedLiteral(clause.clone(), literal.clone());
        // println!("Push propagated literal: {}", element);
        self.elements.push(element);
    }

    pub fn push_model_assignment(&mut self, variable: HConsed<Term>, value: Value) {
        let element = TrailElement::ModelAssignment(variable.clone(), value);
        // println!("Push model assignment: {}", element);
        self.elements.push(element);
        self.model.set_value(variable.get().clone(), value);
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        let removed_element = self.elements.pop();
        if removed_element.is_none() {
            return None;
        }
        // println!("Pop: {}", removed_element.clone().unwrap());

        // If model assignment: clear the variable from the model.
        if let Some(TrailElement::ModelAssignment(var, _)) = &removed_element {
            self.model.clear_value(var.get());
        }
        removed_element
    }

    pub fn last(&self) -> Option<&TrailElement> {
        self.elements.last()
    }

    pub fn value_clause(&self, clause: &Clause) -> Option<bool> {
        let mut found_undefined_literal = false;
        for literal in clause.get_literals() {
            match self.value_literal(literal) {
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

    pub fn value_literal(&self, literal: &Literal) -> Option<bool> {
        // println!("value of literal {}", literal);
        let value_b = self.value_b(literal);
        if value_b.is_none() {
            // println!("value_t {:?}", self.value_t(literal));
            return self.value_t(literal);
        } else {
            // println!("value_b {}", value_b.unwrap());
            return value_b;
        }
    }

    fn value_b(&self, literal: &Literal) -> Option<bool> {
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

    fn value_t(&self, literal: &Literal) -> Option<bool> {
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

    pub fn is_satisfied(&self, clauses: &Vec<Clause>) -> bool {
        if self.is_complete() == false {
            return false;
        }
        for clause in clauses {
            match self.value_clause(clause) {
                Some(false) | None => return false,
                _ => (),
            }
        }
        true
    }

    pub fn is_infeasible(&self) -> bool {
        // Not consistent => infeasible.
        // if !self.is_consistent() {
        //     return true
        // }

        // TODO: trail can be infeasible even if it is consistent.
        unimplemented!()
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
}

impl std::fmt::Display for Trail {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elements: Vec<String> = self.elements.iter().map(|x| x.to_string()).collect();
        write!(fmt, "[{}]", elements.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::clause::Clause;
    use crate::formula::formula::{greater, t};
    use crate::literal::Literal;
    use crate::term::term::{constant, variable};
    use crate::trail::Trail;
    use crate::trail_element::TrailElement;
    use crate::types::value::Value;
    use crate::types::variable::Variable;

    #[test]
    fn test_push_pop() {
        let mut trail = Trail::new();
        let x = variable("x");
        let l = Literal::new(t());
        let c = Clause::new(vec![l.clone()]);
        trail.push_model_assignment(x.clone(), Value::Integer(1));
        trail.push_decided_literal(&l);
        trail.push_propagated_literal(&c, &l);

        assert_eq!(trail.model.get_value(x.get()), Some(&Value::Integer(1)));

        match trail.pop().unwrap() {
            TrailElement::PropagatedLiteral(clause, literal) => {
                assert_eq!(clause, c);
                assert_eq!(literal, l);
            }
            _ => assert!(false),
        }

        assert_eq!(trail.model.get_value(&x), Some(&Value::Integer(1)));

        match trail.pop().unwrap() {
            TrailElement::DecidedLiteral(literal) => {
                assert_eq!(literal, l);
            }
            _ => assert!(false),
        }

        assert_eq!(trail.model.get_value(&x), Some(&Value::Integer(1)));

        match trail.pop().unwrap() {
            TrailElement::ModelAssignment(var, val) => {
                assert_eq!(var, x);
                assert_eq!(val, Value::Integer(1));
            }
            _ => panic!(),
        }

        assert!(trail.pop().is_none());
    }

    #[test]
    fn test_literal_value() {
        // EXAMPLE 1 FROM MCSAT-PAPER
        // M = [x > 1, x ↦ 1, y ↦ 0, z > 0]
        let mut trail = Trail::new();
        trail.push_decided_literal(&Literal::new(greater(
            variable("x"),
            constant(Value::Integer(0)),
        )));
        trail.push_model_assignment(variable("x"), Value::Integer(1));
        trail.push_model_assignment(variable("y"), Value::Integer(0));
        trail.push_decided_literal(&Literal::new(greater(
            variable("z"),
            constant(Value::Integer(0)),
        )));

        assert_eq!(
            trail.value_t(&Literal::new(greater(
                variable("x"),
                constant(Value::Integer(0))
            ))),
            Some(true),
            "expected: value_t(x > 0) == true"
        );
        assert_eq!(
            trail.value_b(&Literal::new(greater(
                variable("x"),
                constant(Value::Integer(0))
            ))),
            Some(true),
            "expected: value_b(x > 0) == true"
        );
        assert_eq!(
            trail.value_t(&Literal::new(greater(
                variable("x"),
                constant(Value::Integer(1))
            ))),
            Some(false),
            "expected: value_t(x > 1) == false"
        );
        assert_eq!(
            trail.value_t(&Literal::new(greater(
                variable("z"),
                constant(Value::Integer(0))
            ))),
            None,
            "expected: value_t(z > 0) == None"
        );
        assert_eq!(
            trail.value_b(&Literal::new(greater(
                variable("z"),
                constant(Value::Integer(0))
            ))),
            Some(true),
            "expected: value_b(z > 0) == true"
        );
    }

    #[test]
    fn test_clause_value() {
        unimplemented!();
    }

    #[test]
    fn test_is_consistent() {
        unimplemented!();
    }

    #[test]
    fn test_is_complete() {
        unimplemented!();
    }

    #[test]
    fn test_is_satisfied() {
        unimplemented!();
    }
}
