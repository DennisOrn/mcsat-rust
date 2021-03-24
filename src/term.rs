#[macro_use]
pub mod term {
    use hashconsing::*;
    pub type Term = HConsed<ActualTerm>;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum ActualTerm {
        Constant(i32),
        Literal(String), //
        Variable(String),
        // True(),
        // False(),

        LessThan(Term, Term),
        GreaterThan(Term, Term),
        Equal(Term, Term),

        Conjunction(Term, Term),
        Disjunction(Term, Term),
        Negation(Term)
    }

    consign! {
        let FACTORY = consign(37) for ActualTerm;
    }

    pub fn constant(value: i32) -> Term {
        FACTORY.mk(ActualTerm::Constant(value))
    }

    pub fn literal(name: &str) -> Term {
        FACTORY.mk(ActualTerm::Literal(String::from(name)))
    }

    pub fn variable(name: &str) -> Term {
        FACTORY.mk(ActualTerm::Variable(String::from(name)))
    }

    pub fn less_than(lhs: Term, rhs: Term) -> Term {
        FACTORY.mk(ActualTerm::LessThan(lhs, rhs))
    }

    pub fn greater_than(lhs: Term, rhs: Term) -> Term {
        FACTORY.mk(ActualTerm::GreaterThan(lhs, rhs))
    }

    pub fn equal(lhs: Term, rhs: Term) -> Term {
        FACTORY.mk(ActualTerm::Equal(lhs, rhs))
    }

    pub fn conjunction(lhs: Term, rhs: Term) -> Term {
        FACTORY.mk(ActualTerm::Conjunction(lhs, rhs))
    }

    pub fn disjunction(lhs: Term, rhs: Term) -> Term {
        FACTORY.mk(ActualTerm::Disjunction(lhs, rhs))
    }

    pub fn negation(term: Term) -> Term {
        FACTORY.mk(ActualTerm::Negation(term))
    }
}

use term::ActualTerm;

impl ::std::fmt::Display for ActualTerm {
    fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ActualTerm::Constant(value) => write!(fmt, "{}", value),
            ActualTerm::Literal(name) => write!(fmt, "{}", name),
            ActualTerm::Variable(name) => write!(fmt, "{}", name),
            ActualTerm::LessThan(lhs, rhs) => write!(fmt, "{} < {}", lhs, rhs),
            ActualTerm::GreaterThan(lhs, rhs) => write!(fmt, "{} > {}", lhs, rhs),
            ActualTerm::Equal(lhs, rhs) => write!(fmt, "{} = {}", lhs, rhs),
            ActualTerm::Conjunction(lhs, rhs) => write!(fmt, "({} ∧ {})", lhs, rhs),
            ActualTerm::Disjunction(lhs, rhs) => write!(fmt, "({} ∨ {})", lhs, rhs),
            ActualTerm::Negation(term) => write!(fmt, "¬{}", term)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Bool(bool),
    Integer(i32)
}