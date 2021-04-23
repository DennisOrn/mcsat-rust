#[macro_use]
pub mod term {
    use crate::model::Model;
    use crate::types::constant::Constant;
    use crate::types::function::Function;
    use crate::types::value::Value;
    use crate::types::variable::Variable;
    use hashconsing::*;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Term {
        Variable(Variable),
        Constant(Constant),
        Function(Function, Vec<HConsed<Term>>), // TODO: Vec or just two args?
    }
    impl Term {
        pub fn evaluate(&self, model: &Model) -> Option<Value> {
            match self {
                Term::Variable(variable) => variable.evaluate(model),
                Term::Constant(constant) => Some(constant.evaluate()),
                Term::Function(function, args) => {
                    // TODO: lazy evaluation?
                    let values: Vec<Value> = args.iter().flat_map(|x| x.evaluate(model)).collect();
                    if values.len() == args.len() {
                        return Some(function.evaluate(model, &values));
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    consign! {
        let FACTORY = consign(37) for Term; // TODO: initial capacity: 37
    }

    pub fn variable(id: &str) -> HConsed<Term> {
        FACTORY.mk(Term::Variable(Variable::new(id)))
    }

    pub fn constant(value: Value) -> HConsed<Term> {
        FACTORY.mk(Term::Constant(Constant::new(value)))
    }

    pub fn plus(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Term> {
        FACTORY.mk(Term::Function(Function::Plus, vec![lhs, rhs]))
    }

    pub fn minus(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Term> {
        FACTORY.mk(Term::Function(Function::Minus, vec![lhs, rhs]))
    }

    impl std::fmt::Display for Term {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Term::Variable(id) => write!(fmt, "{}", id),
                Term::Constant(value) => write!(fmt, "{}", value),
                Term::Function(function, args) => {
                    let args_list: Vec<String> = args.iter().map(|x| x.to_string()).collect();
                    write!(fmt, "({} {})", function.to_string(), args_list.join(" "))
                }
            }
        }
    }
}
