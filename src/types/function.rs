use crate::types::value::Value;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Function {
    // TODO: implement hashconsing for this?
    Plus,
    Minus,
}
impl Function {
    pub fn evaluate(&self, args: &Vec<Value>) -> Value {
        assert!(args.len() == 2);
        match self {
            Function::Plus => args[0] + args[1],
            Function::Minus => args[0] - args[1],
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Function::Plus => write!(fmt, "+"),
            Function::Minus => write!(fmt, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::term::term::{constant, minus, plus};
    use crate::types::value::Value;

    #[test]
    fn test_function_plus_and_minus() {
        let model = Model::new();

        assert_eq!(
            plus(constant(Value::Integer(1)), constant(Value::Integer(2))).evaluate(&model),
            Some(Value::Integer(3))
        );
        assert_eq!(
            minus(constant(Value::Integer(2)), constant(Value::Integer(1))).evaluate(&model),
            Some(Value::Integer(1))
        );
    }
}
