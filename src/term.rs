#[macro_use]
pub mod term {
    use crate::model::Model;
    use hashconsing::*;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Term {
        Variable(Variable),
        Constant(Constant),
        Function(Function, Vec<HConsed<Term>>),
    }
    impl Term {
        // pub fn evaluate<'a>(&self, model: &'a Model) -> Option<&'a Value> {
        //     println!("eval term: {}", self);
        //     let value = model.get_value(self);
        //     value
        // }
        pub fn evaluate(&self, model: &Model) -> Option<Value> {
            println!("eval term: {}", self);
            match self {
                Term::Variable(variable) => variable.evaluate(model),
                Term::Constant(constant) => Some(constant.evaluate()),
                Term::Function(function, args) => function.evaluate(model, args),
            }
        }
    }

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
            println!("eval variable: {}", self);
            match model.get_value(self) {
                Some(value) => Some(value.clone()),
                None => None,
            }
        }
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub struct Constant {
        value: Value,
    }
    impl Constant {
        pub fn evaluate(&self) -> Value {
            println!("eval constant: {}", self);
            self.value
        }
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
    pub enum Value {
        Boolean(bool),
        Integer(i32),
    }

    consign! {
        let FACTORY = consign(37) for Term; // TODO: initial capacity: 37
    }

    pub fn variable(id: &str) -> HConsed<Term> {
        FACTORY.mk(Term::Variable(Variable { id: id.to_string() }))
    }

    pub fn constant(value: Value) -> HConsed<Term> {
        FACTORY.mk(Term::Constant(Constant { value: value }))
    }

    pub fn function(function: Function, args: Vec<HConsed<Term>>) -> HConsed<Term> {
        FACTORY.mk(Term::Function(function, args))
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Function {
        // TODO: implement hashconsing for this?
        Plus,
        Minus,
    }
    impl Function {
        pub fn evaluate(&self, model: &Model, args: &Vec<HConsed<Term>>) -> Option<Value> {
            None
            // match self {
            //     Function::Plus => {
            //         match (args[0].evaluate(model), args[1].evaluate(model)) {
            //             (Some(lhs), Some(rhs)) => lhs + rhs,
            //             _ => None,
            //         }
            //     }
            //     Function::Minus => ,
            // }
        }
    }

    impl std::fmt::Display for Term {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Term::Variable(id) => write!(fmt, "{}", id),
                Term::Constant(value) => write!(fmt, "{}", value),
                Term::Function(function, args) => write!(fmt, "FUN({}, {:?})", function, args),
            }
        }
    }

    impl std::fmt::Display for Variable {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "{}", self.id)
        }
    }

    impl std::fmt::Display for Constant {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "{}", self.value)
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

    impl std::fmt::Display for Value {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Value::Boolean(x) => write!(fmt, "{}", x),
                Value::Integer(x) => write!(fmt, "{}", x),
            }
        }
    }
}
