use std::collections::HashMap;
// use crate::term::term::Expr;
// use crate::term::term::Expression;
// use crate::term::term::Formula;
// use crate::term::term::Term;
// use crate::term::Value;

use crate::experimental::experimental::Expression;
use crate::experimental::experimental::Term;
use crate::experimental::experimental::Value;
use crate::types::boolean::Boolean;
use crate::types::or::Or;

// #[derive(Debug)]
pub struct Model {
    // map_boolean: HashMap<Term, Value> // Does not work
    map_boolean: HashMap<Boolean, Value>
}

impl Model {
    pub fn new() -> Self {
        Model {
            map_boolean: HashMap::new()
        }
    }

    pub fn evaluate<T: Expression>(&self, expression: &T) -> Option<bool> {
        expression.evaluate(self)
    }

    pub fn set_boolean(&mut self, boolean: Boolean, value: Value) {
        self.map_boolean.insert(boolean, value);
    }

    pub fn get_boolean(&self, boolean: &Boolean) -> Option<&Value> {
        return self.map_boolean.get(&boolean);
    }

    pub fn clear_boolean(&mut self, boolean: Boolean) {
        self.map_boolean.remove(&boolean);
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

    // fn check_term_value(&self, t: &Term, value: &Value) -> bool {
    //     match (t, value) {
    //         (Term::Boolean(_), Value::Bool(_)) => true,
    //         (Term::Variable(_), Value::Integer(_)) => true,
    //         (_, _) => false
    //     }
    // }
}