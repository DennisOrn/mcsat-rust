#[macro_use]
pub mod formula {
    use crate::model::Model;
    use crate::term::term::Term;
    use crate::types::predicate::Predicate;
    use hashconsing::*;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Formula {
        True,
        False,
        Predicate(Predicate, Vec<HConsed<Term>>),
    }
    impl Formula {
        pub fn evaluate(&self, model: &Model) -> Option<bool> {
            println!("eval formula\t{}", self);
            match self {
                Formula::True => Some(true),
                Formula::False => Some(false),
                Formula::Predicate(predicate, args) => predicate.evaluate(model, args),
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

    pub fn predicate(predicate: Predicate, args: Vec<HConsed<Term>>) -> HConsed<Formula> {
        FACTORY.mk(Formula::Predicate(predicate, args))
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
