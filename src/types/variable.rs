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
        print!("eval variable\t{}", self);
        match model.get_value(self) {
            Some(value) => {
                println!(": {}", value);
                Some(value.clone())
            }
            None => {
                println!(": None");
                None
            }
        }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.id)
    }
}
