use crate::experimental::experimental::Expression;
use crate::experimental::experimental::Term;
use crate::experimental::experimental::Value;
use crate::model::Model;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Boolean {
    id: String
}

impl Expression for Boolean {
    fn evaluate(&self, model: &Model) -> Option<bool> {
        println!("evaluating: Boolean");
        match model.get_boolean(self) {
            Some(Value::Bool(true))  => Some(true),
            Some(Value::Bool(false)) => Some(false),
            _                        => None
        }
    }
}

impl Term for Boolean {}

pub fn new(id: &str) -> Boolean {
    Boolean { id: id.to_string()}
}