use crate::clause::Clause;
use crate::formula::formula::{equal, Formula};
use crate::literal::Literal;
use crate::term::term::{constant, variable, Term};
use crate::trail::Trail;
use crate::types::function::Function;
use crate::types::predicate::Predicate;
use crate::types::value::Value;
use hashconsing::HConsed;
use std::collections::VecDeque;

pub trait Theory {
    fn propagate(&self);
    fn decide(&self, variable: &HConsed<Term>, trail: &Trail) -> Option<Value>;
    fn conflict(&self, variables: &VecDeque<HConsed<Term>>, trail: &Trail) -> Option<Clause>;
    fn consume(&self);
    fn backjump_decide(&self);

    // fn explain(&self) -> Clause;
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

    // pub fn var(&self, id: &str) -> HConsed<Formula> {
    //     equal(variable(id), constant(Value::True))
    // }

    pub fn _true(&self) -> HConsed<Term> {
        constant(Value::True)
    }

    pub fn _false(&self) -> HConsed<Term> {
        constant(Value::False)
    }

    pub fn _eq(&self, lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        equal(lhs, rhs)
    }

    pub fn _var(&self, id: &str) -> HConsed<Term> {
        variable(id)
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
                // debug!("Push model assignment: {} = {}", variable, value);
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

    fn conflict(&self, variables: &VecDeque<HConsed<Term>>, trail: &Trail) -> Option<Clause> {
        for variable in variables {
            // Construct two literals, var = true and var = false, and check their values.
            // If both are true, the trail is infeasible.
            let literal_true = Literal::new(equal(variable.clone(), constant(Value::True)));
            let literal_false = Literal::new(equal(variable.clone(), constant(Value::False)));
            let literal_true_value = trail.value_literal(&literal_true);
            let literal_false_value = trail.value_literal(&literal_false);

            match (literal_true_value, literal_false_value) {
                (Some(true), Some(true)) => {
                    let explanation =
                        Clause::new(vec![literal_true.negate(), literal_false.negate()]);
                    return Some(explanation);
                }
                _ => (),
            }
        }

        None
    }

    fn consume(&self) {
        unimplemented!()
    }

    fn backjump_decide(&self) {
        unimplemented!()
    }

    // fn explain(&self) -> Clause {
    //     unimplemented!()
    // }
}
