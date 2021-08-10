pub mod clause;
mod formula;
pub mod literal;
mod model;
pub mod solver;
mod state;
mod term;
pub mod theory;
mod trail;
mod trail_element;
mod types;

use crate::clause::Clause;
use crate::literal::Literal;
use crate::solver::Solver;
use crate::theory::BooleanTheory;

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

use std::collections::VecDeque;

/**
Set the log level to adjust tha amount of information that is printed.
From low to high priority: Trace / Debug / Info / Warn / Error
*/
pub fn configure_logger(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, level)
        .init();
}

pub fn example_solver_unsat() -> Solver {
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
    let solver = Solver::new(Box::new(t), clauses, undecided);

    solver
}
