use crate::experimental::experimental::Expression;
use crate::model::Model;

pub struct Or {
    operands: Vec<Box<dyn Expression>>
}

impl Expression for Or {
    fn evaluate(&self, model: &Model) -> Option<bool> {
        println!("evaluating: Or");

        let mut none_count = 0;
        for operand in &self.operands {
            match operand.evaluate(model) {
                Some(true) => return Some(true),
                None => none_count += 1,
                _ => ()
            }
        }
        if none_count > 0 {
            None
        } else {
            Some(false)
        }
    }
}

pub fn new(operands: Vec<Box<dyn Expression>>) -> Or {
    Or { operands: operands }
}