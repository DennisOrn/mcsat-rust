use crate::term::term::Term;
use crate::types::value::Value;
use crate::types::variable::Variable;
use hashconsing::HConsed;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Model {
    map: HashMap<Term, Value>, // TODO: should 'static be used here?
}

impl Model {
    pub fn new() -> Self {
        Model {
            map: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, t: Term, value: Value) {
        // TODO: add assertions.
        self.map.insert(t, value);
    }

    pub fn get_value(&self, t: &Term) -> Option<&Value> {
        self.map.get(t)
    }

    pub fn clear_value(&mut self, t: &Term) {
        self.map.remove(t);
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::term::term::variable;
    use crate::types::value::Value;

    #[test]
    fn test_model() {
        let mut model = Model::new();

        assert_eq!(model.get_value(&variable("x")), None);

        let x = variable("x");
        model.set_value(x.get().clone(), Value::Integer(5));

        assert_eq!(model.get_value(&variable("x")), Some(&Value::Integer(5)));

        model.clear_value(variable("x").get());

        assert_eq!(model.get_value(&variable("x")), None);
    }
}
