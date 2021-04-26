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
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::types::value::Value;
    use crate::types::variable::Variable;

    #[test]
    fn test_model() {
        let mut model = Model::new();

        assert_eq!(model.get_value(&Variable::new("x")), None);

        model.set_value(Variable::new("x"), Value::Integer(5));

        assert_eq!(
            model.get_value(&Variable::new("x")),
            Some(&Value::Integer(5))
        );

        model.clear_value(Variable::new("x"));

        assert_eq!(model.get_value(&Variable::new("x")), None);
    }
}
