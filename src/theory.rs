use crate::clause::Clause;
use crate::formula::formula::equal;
use crate::formula::formula::Formula;
use crate::literal::Literal;
use crate::term::term::{constant, variable, Term};
use crate::trail::Trail;
use crate::types::function::Function;
use crate::types::predicate::Predicate;
use crate::types::value::Value;
use crate::types::variable::Variable;
use hashconsing::HConsed;

pub trait Theory {
    fn propagate(&self);
    fn decide(&self, variable: &HConsed<Term>, trail: &Trail) -> Option<Value>;
    fn conflict(&self) -> Clause;
    fn consume(&self);
    fn backjump_decide(&self);

    fn explain(&self) -> Clause;
}

pub struct BooleanTheory {
    predicates: Vec<Predicate>,
    functions: Vec<Function>,
    values: Vec<Value>,
}

impl BooleanTheory {
    pub fn new() -> BooleanTheory {
        BooleanTheory {
            predicates: vec![Predicate::Equal],
            functions: vec![],
            values: vec![Value::True, Value::False],
        }
    }

    pub fn var(&self, id: &str) -> HConsed<Formula> {
        equal(variable(id), constant(Value::True))
    }
}

impl Theory for BooleanTheory {
    fn propagate(&self) {
        unimplemented!()
    }

    fn decide(&self, variable: &HConsed<Term>, trail: &Trail) -> Option<Value> {
        // Create clone of trail to avoid pushing and popping on the "real" trail.
        let mut trail_clone = trail.clone();
        for value in &self.values {
            trail_clone.push_model_assignment(variable.clone(), *value);
            if trail_clone.is_consistent() {
                // Commit model assignment and return.
                // println!("Push model assignment: {} = {}", variable, value);
                // trail.push_model_assignment(variable, *value);
                return Some(*value);
            } else {
                let _ = trail_clone.pop();
            }
        }
        None

        // for value in &self.values {
        //     let literal = Literal::new(
        //         equal(variable.clone(), constant(*value)),
        //         // vec![variable.clone()],
        //         false,
        //     );
        //     match trail.value_literal(&literal) {
        //         Some(true) | None => {
        //             return Some(*value);
        //         }
        //         _ => (),
        //     }
        // }
        // None
    }

    fn conflict(&self) -> Clause {
        self.explain()
    }

    fn consume(&self) {
        unimplemented!()
    }

    fn backjump_decide(&self) {
        unimplemented!()
    }

    fn explain(&self) -> Clause {
        // unimplemented!()
        // TODO: this is hardcoded for now.
        Clause::new(vec![
            Literal::new(equal(variable("y"), constant(Value::True)), true),
            Literal::new(equal(variable("y"), constant(Value::False)), true),
        ])
    }
}
