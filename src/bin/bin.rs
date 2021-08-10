use mcsat_rust::configure_logger;

use mcsat_rust::clause::Clause;
use mcsat_rust::literal::Literal;
use mcsat_rust::solver::Solver;
use mcsat_rust::theory::BooleanTheory;

use colored::*;
use log::LevelFilter;

use std::collections::VecDeque;

pub fn main() {
    configure_logger(LevelFilter::Debug);
    example_unsat()
}

fn example_unsat() {
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
    let undecided = VecDeque::from(vec![t._var("x"), t._var("y")]);
    let mut solver = Solver::new(Box::new(t), clauses, undecided);

    // match solver.run_hardcoded() {
    match solver.run() {
        true => println!("{}", "SAT".green()),
        false => println!("{}", "UNSAT".red()),
    }
}
