use crate::term::term::Expr;
use crate::term::term::Term;
use crate::term::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Model {
    map: HashMap<Term, Value>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            map: HashMap::new(),
        }
    }

    // pub fn evaluate(&self, e: &Expr) -> Option<bool> {
    //     // println!("EVAL: {}", e);
    //     match e.get() {
    //         Expression::Term(term) => self.evaluate_term(term),
    //         Expression::Formula(formula) => self.evaluate_formula(formula)
    //     }
    // }

    // fn evaluate_term(&self, t: &Term) -> Option<bool> {
    //     match t {
    //         Term::Boolean(_) => {
    //             if let Some(value) = self.map.get(&t) {
    //                 if let Value::Bool(b) = value {
    //                     return Some(*b)
    //                 }
    //                 return None
    //             }
    //             return None
    //         }
    //         Term::True() => {
    //             Some(true)
    //         }
    //         Term::False() => {
    //             Some(false)
    //         }
    //         _ => panic!()
    //     }
    // }

    // fn evaluate_formula(&self, f: &Formula) -> Option<bool> {
    //     match f {
    //         Formula::Negation(e) => {
    //             match self.evaluate(e) {
    //                 Some(true) => Some(false),
    //                 Some(false) => Some(true),
    //                 _ => None
    //             }
    //         }
    //         Formula::Disjunction(lhs, rhs) => {
    //             match (self.evaluate(lhs), self.evaluate(rhs)) { // TODO: evaluate lhs first and check result
    //                 (Some(true), _) => return Some(true),
    //                 (_, Some(true)) => return Some(true),
    //                 (None, _) => return None,
    //                 (_, None) => return None,
    //                 (_, _) => return Some(false)
    //             }
    //         }
    //         _ => panic!()
    //     }
    // }

    pub fn set_value(&mut self, t: Term, value: Value) {
        // assert!(self.check_term_value(&t, &value));
        self.map.insert(t, value);
    }

    pub fn clear_value(&mut self, t: Term) {
        self.map.remove(&t);
    }

    // fn check_term_value(&self, t: &Term, value: &Value) -> bool {
    //     match (t, value) {
    //         (Term::Boolean(_), Value::Bool(_)) => true,
    //         (Term::Variable(_), Value::Integer(_)) => true,
    //         (_, _) => false
    //     }
    // }
}
