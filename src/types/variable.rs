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
}

impl std::fmt::Display for Variable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::term::term::variable;
    use crate::types::value::Value;

    #[test]
    fn test_evaluate_undefined_variable() {
        let model = Model::new();

        assert_eq!(variable("x").evaluate(&model), None);
    }

    #[test]
    fn test_evaluate_defined_variable() {
        let mut model = Model::new();
        let x = variable("x");
        model.set_value(x.get().clone(), Value::Integer(5));

        assert_eq!(variable("x").evaluate(&model), Some(Value::Integer(5)));
    }
}
