#[macro_use]
pub mod term {
    use hashconsing::*;
    pub type Expr = HConsed<Expression>;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Expression {
        Term(Term),
        Formula(Formula)
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Term {
        Constant(i32),
        Boolean(String),
        Variable(String),
        True(),
        False()
    }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum Formula {
        LessThan(Expr, Expr),
        GreaterThan(Expr, Expr),
        Equal(Expr, Expr),

        Conjunction(Expr, Expr),
        Disjunction(Expr, Expr),
        Negation(Expr)
    }

    use Term::*;
    use Formula::*;

    consign! {
        let FACTORY = consign(37) for Expression; // TODO: what does 37 mean?
    }

    pub fn constant(value: i32) -> Expr {
        FACTORY.mk(Expression::Term(Constant(value)))
    }

    pub fn boolean(name: &str) -> Expr {
        FACTORY.mk(Expression::Term(Boolean(String::from(name))))
    }

    pub fn variable(name: &str) -> Expr {
        FACTORY.mk(Expression::Term(Variable(String::from(name))))
    }

    pub fn t() -> Expr {
        FACTORY.mk(Expression::Term(True()))
    }

    pub fn f() -> Expr {
        FACTORY.mk(Expression::Term(False()))
    }

    pub fn less_than(lhs: Expr, rhs: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(LessThan(lhs, rhs)))
    }

    pub fn greater_than(lhs: Expr, rhs: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(GreaterThan(lhs, rhs)))
    }

    pub fn equal(lhs: Expr, rhs: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(Equal(lhs, rhs)))
    }

    pub fn conjunction(lhs: Expr, rhs: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(Conjunction(lhs, rhs)))
    }

    pub fn disjunction(lhs: Expr, rhs: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(Disjunction(lhs, rhs)))
    }

    pub fn negation(term: Expr) -> Expr {
        FACTORY.mk(Expression::Formula(Negation(term)))
    }
}

use term::Expression;
use term::Term;
use term::Formula;

impl ::std::fmt::Display for Expression {
    fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Expression::Term(term)       => write!(fmt, "{}", term),
            Expression::Formula(formula) => write!(fmt, "{}", formula)
        }
    }
}

impl ::std::fmt::Display for Term {
    fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Term::Constant(value) => write!(fmt, "{}", value),
            Term::Boolean(name)   => write!(fmt, "{}", name),
            Term::Variable(name)  => write!(fmt, "{}", name),
            Term::True()          => write!(fmt, "true"),
            Term::False()         => write!(fmt, "false")
        }
    }
}

impl ::std::fmt::Display for Formula {
    fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Formula::LessThan(lhs, rhs)    => write!(fmt, "{} < {}", lhs, rhs),
            Formula::GreaterThan(lhs, rhs) => write!(fmt, "{} > {}", lhs, rhs),
            Formula::Equal(lhs, rhs)       => write!(fmt, "{} = {}", lhs, rhs),
            Formula::Conjunction(lhs, rhs) => write!(fmt, "({} ∧ {})", lhs, rhs),
            Formula::Disjunction(lhs, rhs) => write!(fmt, "({} ∨ {})", lhs, rhs),
            Formula::Negation(term)        => write!(fmt, "¬{}", term)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Bool(bool),
    Integer(i32)
}