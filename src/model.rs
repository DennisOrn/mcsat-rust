use crate::types::value::Value;
use crate::types::variable::Variable;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Model {
    map: HashMap<Variable, Value>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            map: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, t: Variable, value: Value) {
        // assert!(self.check_term_value(&t, &value));
        self.map.insert(t, value);
    }

    pub fn get_value(&self, t: &Variable) -> Option<&Value> {
        self.map.get(t)
    }

    pub fn clear_value(&mut self, t: Variable) {
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
