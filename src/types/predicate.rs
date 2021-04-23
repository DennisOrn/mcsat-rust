use crate::types::value::Value;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Predicate {
    // TODO: implement hashconsing for this?
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
}
impl Predicate {
    pub fn evaluate(&self, args: &Vec<Value>) -> bool {
        assert!(args.len() == 2);
        match self {
            Predicate::Less => args[0] < args[1],
            Predicate::LessEqual => args[0] <= args[1],
            Predicate::Greater => args[0] > args[1],
            Predicate::GreaterEqual => args[0] >= args[1],
            Predicate::Equal => args[0] == args[1],
        }
    }
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Predicate::Less => write!(fmt, "<"),
            Predicate::Greater => write!(fmt, ">"),
            Predicate::LessEqual => write!(fmt, "≤"),
            Predicate::GreaterEqual => write!(fmt, "≥"),
            Predicate::Equal => write!(fmt, "="),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::formula::formula::{equal, greater, greater_equal, less, less_equal};
    use crate::model::Model;
    use crate::term::term::constant;
    use crate::types::value::Value;

    #[test]
    fn test_predicate_less() {
        let model = Model::new();

        assert_eq!(
            less(constant(Value::Integer(1)), constant(Value::Integer(2))).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            less(constant(Value::Integer(2)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            less(constant(Value::Integer(3)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
    }

    #[test]
    fn test_predicate_less_equal() {
        let model = Model::new();

        assert_eq!(
            less_equal(constant(Value::Integer(1)), constant(Value::Integer(2))).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            less_equal(constant(Value::Integer(2)), constant(Value::Integer(2))).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            less_equal(constant(Value::Integer(3)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
    }

    #[test]
    fn test_predicate_greater() {
        let model = Model::new();

        assert_eq!(
            greater(constant(Value::Integer(1)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            greater(constant(Value::Integer(2)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            greater(constant(Value::Integer(3)), constant(Value::Integer(2))).evaluate(&model),
            Some(true)
        );
    }

    #[test]
    fn test_predicate_greater_equal() {
        let model = Model::new();

        assert_eq!(
            greater_equal(constant(Value::Integer(1)), constant(Value::Integer(2)))
                .evaluate(&model),
            Some(false)
        );
        assert_eq!(
            greater_equal(constant(Value::Integer(2)), constant(Value::Integer(2)))
                .evaluate(&model),
            Some(true)
        );
        assert_eq!(
            greater_equal(constant(Value::Integer(3)), constant(Value::Integer(2)))
                .evaluate(&model),
            Some(true)
        );
    }

    #[test]
    fn test_predicate_equal() {
        let model = Model::new();

        assert_eq!(
            equal(constant(Value::Integer(1)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
        assert_eq!(
            equal(constant(Value::Integer(2)), constant(Value::Integer(2))).evaluate(&model),
            Some(true)
        );
        assert_eq!(
            equal(constant(Value::Integer(3)), constant(Value::Integer(2))).evaluate(&model),
            Some(false)
        );
    }
}
