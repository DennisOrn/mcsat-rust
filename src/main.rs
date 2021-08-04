mod clause;
mod formula;
mod literal;
mod model;
mod solver;
mod state;
mod term;
mod theory;
mod trail;
mod trail_element; // TODO: underscore?
mod types;

use crate::clause::Clause;
use crate::literal::Literal;
use crate::solver::Solver;
use crate::term::term::Term;
use crate::theory::BooleanTheory;
use colored::*;
use hashconsing::HConsed;
use std::collections::VecDeque;

fn main() {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._true())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._false())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._true())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._false())),
    ]);

    let clauses = vec![clause1, clause2, clause3, clause4];
    let undecided: VecDeque<HConsed<Term>> = VecDeque::from(vec![t._var("x"), t._var("y")]);

    for c in &clauses {
        println!("{}", c);
    }
    println!("\n");

    let mut solver = Solver::new(Box::new(t), clauses, undecided /*, map*/);
    // match solver.run() {
    match solver.run_hardcoded() {
        true => println!("{}", "SAT".green()),
        false => println!("{}", "UNSAT".red()),
    }
}
