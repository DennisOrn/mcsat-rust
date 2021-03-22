use std::collections::HashMap;
use crate::term::term::Term;
use crate::term::term::ActualTerm;
use crate::term::Value;

#[derive(Debug)]
pub struct Model {
    map: HashMap<Term, Value>
}

impl Model {
    pub fn new() -> Self {
        Model { map: HashMap::new() }
    }

    pub fn evaluate(&self, t: &Term) -> Option<bool> {
        // println!("recursive evaluate: {}", t);
        match t.get() {
            ActualTerm::Literal(_) => {
                if let Some(value) = self.map.get(&t) {
                    if let Value::Bool(b) = value {
                        return Some(*b)
                    }
                    return None
                }
                return None

                // match self.map.get(&t) {
                //     None => None,
                //     Some(value) => {
                //         if let Value::Bool(b) = value {
                //             Some(*b)
                //         } else {
                //             None
                //         }
                //     }
                // }
            }
            ActualTerm::Negation(term) => {
                match self.evaluate(term) {
                    Some(true) => Some(false),
                    Some(false) => Some(true),
                    _ => None
                }
            }
            ActualTerm::Disjunction(lhs, rhs) => {
                match (self.evaluate(lhs), self.evaluate(rhs)) {
                    (Some(true), _) => return Some(true),
                    (_, Some(true)) => return Some(true),
                    (None, _) => return None,
                    (_, None) => return None,
                    (_, _) => return Some(false)
                }
            }
            _ => panic!()
        }
    }

    pub fn set_value(&mut self, t: Term, value: Value) {
        self.map.insert(t, value);
    }

    pub fn clear_value(&mut self, t: Term) {
        self.map.remove(&t);
    }
}