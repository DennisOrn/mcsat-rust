#[macro_use]
pub mod formula {
    use crate::term::term::Term;
    use hashconsing::*;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Formula {
        True,
        False,
        Predicate(Predicate, Vec<HConsed<Term>>),
    }

    consign! {
        let FACTORY = consign(37) for Formula; // TODO: what does 37 mean?
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

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Predicate {
        // TODO: implement hashconsing for this?
        LessThan,
        GreaterThan,
        LessThanOrEqual,
        GreaterThanOrEqual,
        Equal,
    }

    impl ::std::fmt::Display for Formula {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                Formula::True => write!(fmt, "True"),
                Formula::False => write!(fmt, "False"),
                Formula::Predicate(predicate, args) => {
                    match write!(fmt, "({}", predicate) {
                        Err(e) => return Err(e),
                        _      => (),
                    }
                    for arg in args {
                        match write!(fmt, " {}", arg) {
                            Err(e) => return Err(e),
                            _      => (),
                        }
                    }
                    match write!(fmt, ")") {
                        Err(e) => Err(e),
                        _      => Ok(()),
                    }
                }
            }
        }
    }

    impl ::std::fmt::Display for Predicate {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                Predicate::LessThan           => write!(fmt, "<"),
                Predicate::GreaterThan        => write!(fmt, ">"),
                Predicate::LessThanOrEqual    => write!(fmt, "≤"),
                Predicate::GreaterThanOrEqual => write!(fmt, "≥"),
                Predicate::Equal              => write!(fmt, "="),
            }
        }
    }
}
