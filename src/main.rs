mod clause;
mod decidable;
mod solver;
mod state;
mod theory;
mod trail;
mod variable;

use crate::clause::Clause;
use crate::solver::Solver;
use crate::theory::Boolean;
use crate::variable::Variable;

fn main() {

    hashcons_test()

    // let clause1 = Clause::new( vec![Variable::new(1, Boolean::Undef)] );
    // let clause2 = Clause::new( vec![Variable::new(2, Boolean::Undef)] );
    // let clause3 = Clause::new( vec![Variable::new(3, Boolean::Undef)] );

    // let clauses = vec![clause1, clause2, clause3];
    // print_clauses(&clauses);

    // let undecided = vec![1, 2, 3];

    // let mut solver = Solver::new(clauses, undecided);
    // match solver.solve() {
    //     true => println!("SAT"),
    //     false => println!("UNSAT"),
    // }
}

// fn print_clauses(clauses: &Vec<Clause>) {
//     println!("CLAUSES");
//     for clause in clauses {
//         println!("{:?}", clause);
//     }
//     println!();
// }


// use hashconsing:: { HConsed, HashConsign, HConsign };

// type Term = HConsed<ActualTerm>;
// #[derive(Debug, Hash, Clone, PartialEq, Eq)]
// enum ActualTerm {
//     Var(usize),
//     Lam(Term),
//     App(Term, Term)
// }
// use ActualTerm::*;

#[macro_use]
extern crate hashconsing;

pub mod term {
    use hashconsing::{ HConsed, HashConsign };
    pub type Term = HConsed<ActualTerm>;

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub enum ActualTerm {
        // Var(String, Option<i32>),
        // Lam(Term),
        // App(Term, Term),

        Constant(i32),
        Literal(String),
        Variable(String),

        Conjunction(Term, Term),
        Disjunction(Term, Term),
        Implication(Term, Term),

        LessThan(Term, Term),
        GreaterThan(Term, Term),
        Equal(Term, Term)

        // Assignment(, Term)
    }

    consign! {
        let factory = consign(37) for ActualTerm;
    }

    pub fn constant(value: i32) -> Term {
        factory.mk(ActualTerm::Constant(value))
    }

    pub fn literal(name: &str) -> Term {
        factory.mk(ActualTerm::Literal(String::from(name)))
    }

    pub fn variable(name: &str) -> Term {
        factory.mk(ActualTerm::Variable(String::from(name)))
    }

    pub fn less_than(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::LessThan(lhs, rhs))
    }

    pub fn greater_than(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::GreaterThan(lhs, rhs))
    }

    pub fn equal(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::Equal(lhs, rhs))
    }

    pub fn conjunction(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::Conjunction(lhs, rhs))
    }

    pub fn disjunction(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::Disjunction(lhs, rhs))
    }

    pub fn implication(lhs: Term, rhs: Term) -> Term {
        factory.mk(ActualTerm::Implication(lhs, rhs))
    }
}

use term::ActualTerm;

impl ::std::fmt::Display for ActualTerm {
    fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            // ActualTerm::Var(name, value) => write!(fmt, "{{{},{:?}}}", name, value),
            // ActualTerm::Lam(t) => write!(fmt, "({})", t.get()),
            // ActualTerm::App(u, v) => write!(fmt, "{}.{}", u.get(), v.get())
            ActualTerm::Constant(value) => write!(fmt, "{}", value),
            ActualTerm::Literal(name) => write!(fmt, "{}", name),
            ActualTerm::Variable(name) => write!(fmt, "{}", name),
            ActualTerm::LessThan(lhs, rhs) => write!(fmt, "{} < {}", lhs, rhs),
            ActualTerm::GreaterThan(lhs, rhs) => write!(fmt, "{} > {}", lhs, rhs),
            ActualTerm::Equal(lhs, rhs) => write!(fmt, "{} = {}", lhs, rhs),
            ActualTerm::Conjunction(lhs, rhs) => write!(fmt, "({} ∧ {})", lhs, rhs),
            ActualTerm::Disjunction(lhs, rhs) => write!(fmt, "({} ∨ {})", lhs, rhs),
            ActualTerm::Implication(lhs, rhs) => write!(fmt, "({} → {})", lhs, rhs)
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Int(i32),
    Bool(bool),
    // None // TODO: not needed, hashmap returns None if key doesn't exist (?)
}

fn hashcons_test() {
    // x < 1 ∨ p, ¬p ∨ x = 2

    // TODO: how do I change values?
    // Separate data structure?

    let x = term::variable("x");
    let constant_1 = term::constant(1);
    let less_than = term::less_than(x, constant_1);
    let p = term::literal("p");
    let disjunction_1 = term::disjunction(less_than, p);
    println!("constructed clause: {}", disjunction_1);

    let mut hash_map: HashMap<term::Term, Value> = HashMap::new();

    // hash_map.insert(term::variable("x"), Value::None);
    // hash_map.insert(term::variable("p"), Value::None);
    hash_map.insert(term::variable("1"), Value::Int(1));
    // for (key, value) in &hash_map {
    //     println!("{:?}: {:?}", key, value);
    // }

    println!();
    let mut result = evaluate(&disjunction_1, &hash_map);
    // println!("result: {}", result);
}

// TODO: return Option<bool>?
fn evaluate(t: &term::Term, hash_map: &HashMap<term::Term, Value>) -> bool {
    match t.get() {
        ActualTerm::LessThan(lhs, rhs) => {
            println!("LessThan({}, {})", lhs, rhs);
            return lhs < rhs

            // if let Some(l) = hash_map.get(lhs) {
            //     println!("{:?}", l);
            //     if let Some(r) = hash_map.get(rhs) {
            //         println!("{:?}", r);
            //         return l < r;
            //     }
            // }
            // return false
        },
        ActualTerm::GreaterThan(lhs, rhs) => {
            println!("GreaterThan({}, {})", lhs, rhs);
            return lhs > rhs
        },
        ActualTerm::Equal(lhs, rhs) => {
            println!("Equal({}, {})", lhs, rhs);
            return lhs == rhs
        },
        ActualTerm::Conjunction(lhs, rhs) => {
            println!("Conjunction({}, {})", lhs, rhs);
            return evaluate(lhs, hash_map) && evaluate(rhs, hash_map)
        },
        ActualTerm::Disjunction(lhs, rhs) => {
            println!("Disjunction({}, {})", lhs, rhs);
            return evaluate(lhs, hash_map) || evaluate(rhs, hash_map)
        },
        _ => panic!("This is not supposed to happen...")
    }
}
