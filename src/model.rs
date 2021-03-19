use std::collections::HashMap;
use crate::term::term::Term;
use crate::term::Value;

#[derive(Debug)]
pub struct Model {
    map: HashMap<Term, Value>
}

impl Model {
    pub fn new() -> Self {
        Model { map: HashMap::new() }
    }

    pub fn evaluate(t: Term) -> Option<bool> {
        unimplemented!()
    }

    pub fn set_value(&mut self, t: Term, value: Value) {
        self.map.insert(t, value);
    }

    pub fn clear_value(&mut self, t: Term) {
        self.map.remove(&t);
    }
}