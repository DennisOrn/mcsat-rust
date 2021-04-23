use crate::model::Model;
use crate::types::value::Value;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Variable {
    id: String, // TODO: usize is probably more efficient than String.
                // TODO: add member indicating whether variable is negated?
}
impl Variable {
    pub fn new(id: &str) -> Variable {
        Variable { id: id.to_string() }
    }
    pub fn evaluate(&self, model: &Model) -> Option<Value> {
        match model.get_value(self) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::types::value::Value;
    use crate::types::variable::Variable;

    #[test]
    fn test_evaluate_undefined_variable() {
        let model = Model::new();

        assert_eq!(Variable::new("x").evaluate(&model), None);
    }

    #[test]
    fn test_evaluate_defined_variable() {
        let mut model = Model::new();
        model.set_value(Variable::new("x"), Value::Integer(5));

        assert_eq!(Variable::new("x").evaluate(&model), Some(Value::Integer(5)));
    }
}
