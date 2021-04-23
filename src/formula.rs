#[macro_use]
pub mod formula {
    use crate::model::Model;
    use crate::term::term::Term;
    use crate::types::predicate::Predicate;
    use crate::types::value::Value;
    use hashconsing::*;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Formula {
        True,
        False,
        Predicate(Predicate, Vec<HConsed<Term>>), // TODO: Vec or just two args?
    }
    impl Formula {
        pub fn evaluate(&self, model: &Model) -> Option<bool> {
            match self {
                Formula::True => Some(true),
                Formula::False => Some(false),
                Formula::Predicate(predicate, args) => {
                    // TODO: lazy evaluation?
                    let values: Vec<Value> = args.iter().flat_map(|x| x.evaluate(model)).collect();
                    if values.len() == args.len() {
                        return Some(predicate.evaluate(model, &values));
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    consign! {
        let FACTORY = consign(37) for Formula; // TODO: initial capacity: 37
    }

    pub fn t() -> HConsed<Formula> {
        FACTORY.mk(Formula::True)
    }

    pub fn f() -> HConsed<Formula> {
        FACTORY.mk(Formula::False)
    }

    pub fn less(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(Predicate::Less, vec![lhs, rhs]))
    }

    pub fn less_equal(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(Predicate::LessEqual, vec![lhs, rhs]))
    }

    pub fn greater(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(Predicate::Greater, vec![lhs, rhs]))
    }

    pub fn greater_equal(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(Predicate::GreaterEqual, vec![lhs, rhs]))
    }

    pub fn equal(lhs: HConsed<Term>, rhs: HConsed<Term>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(Predicate::Equal, vec![lhs, rhs]))
    }

    impl std::fmt::Display for Formula {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Formula::True => write!(fmt, "True"),
                Formula::False => write!(fmt, "False"),
                Formula::Predicate(predicate, args) => {
                    let args_list: Vec<String> = args.iter().map(|x| x.to_string()).collect();
                    write!(fmt, "({} {})", predicate.to_string(), args_list.join(" "))
                }
            }
        }
    }
}
