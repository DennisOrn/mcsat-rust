#[macro_use]
pub mod term {
    use hashconsing::*;
    pub type Expr = HConsed<Term>; // TODO: remove this

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Term {
        Variable(Variable),
        Constant(Constant),
        Function(Function, Vec<HConsed<Term>>),
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub struct Variable {
        id: String, // TODO: usize is probably more efficient than String.
                    // TODO: add member indicating whether variable is negated?
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub struct Constant {
        value: Value,
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Value {
        Bool(bool),
        Integer(i32),
    }

    consign! {
        let FACTORY = consign(37) for Term; // TODO: what does 37 mean?
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

    impl ::std::fmt::Display for Term {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                Term::Variable(id) => write!(fmt, "{}", id),
                Term::Constant(value) => write!(fmt, "{}", value),
                Term::Function(function, args) => write!(fmt, "FUN({}, {:?})", function, args),
            }
        }
    }

    impl ::std::fmt::Display for Variable {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(fmt, "{}", self.id)
        }
    }

    impl ::std::fmt::Display for Constant {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(fmt, "{}", self.value)
        }
    }

    impl ::std::fmt::Display for Function {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                Function::Plus => write!(fmt, "+"),
                Function::Minus => write!(fmt, "-"),
            }
        }
    }

    impl ::std::fmt::Display for Value {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                Value::Bool(x)    => write!(fmt, "{}", x),
                Value::Integer(x) => write!(fmt, "{}", x),
            }
        }
    }
}