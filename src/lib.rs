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

pub fn example_solver_unsat_1() -> Solver {
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
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_unsat_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._true())),
        Literal::new(t._eq(t._var("z"), t._true())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._true())),
        Literal::new(t._eq(t._var("z"), t._false())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._false())),
        Literal::new(t._eq(t._var("z"), t._true())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._false())),
        Literal::new(t._eq(t._var("z"), t._false())),
    ]);
    let clause5 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._true())),
        Literal::new(t._eq(t._var("z"), t._true())),
    ]);
    let clause6 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._true())),
        Literal::new(t._eq(t._var("z"), t._false())),
    ]);
    let clause7 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._false())),
        Literal::new(t._eq(t._var("z"), t._true())),
    ]);
    let clause8 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._false())),
        Literal::new(t._eq(t._var("z"), t._false())),
    ]);

    let clauses = vec![
        clause1, clause2, clause3, clause4, clause5, clause6, clause7, clause8,
    ];
    let undecided = VecDeque::from(vec![t._var("x"), t._var("y"), t._var("z")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_sat_1() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._true())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._false())),
    ]);

    let clauses = vec![clause1, clause2];
    let undecided = VecDeque::from(vec![t._var("x"), t._var("y")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_sat_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![Literal::new(t._eq(t._var("x"), t._true()))]);

    let clauses = vec![clause1];
    let undecided = VecDeque::from(vec![t._var("x")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_pigeonhole_1() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![Literal::new(t._eq(t._var("one"), t._true()))]);
    let clause2 = Clause::new(vec![Literal::new(t._eq(t._var("two"), t._true()))]);
    let clause3 = Clause::new(vec![
        Literal::new(t._eq(t._var("one"), t._false())),
        Literal::new(t._eq(t._var("two"), t._false())),
    ]);
    let clauses = vec![clause1, clause2, clause3];
    let undecided = VecDeque::from(vec![t._var("one"), t._var("two")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_pigeonhole_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t._eq(t._var("one"), t._true())),
        Literal::new(t._eq(t._var("two"), t._true())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t._eq(t._var("three"), t._true())),
        Literal::new(t._eq(t._var("four"), t._true())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t._eq(t._var("five"), t._true())),
        Literal::new(t._eq(t._var("six"), t._true())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t._eq(t._var("one"), t._false())),
        Literal::new(t._eq(t._var("three"), t._false())),
    ]);
    let clause5 = Clause::new(vec![
        Literal::new(t._eq(t._var("one"), t._false())),
        Literal::new(t._eq(t._var("five"), t._false())),
    ]);
    let clause6 = Clause::new(vec![
        Literal::new(t._eq(t._var("three"), t._false())),
        Literal::new(t._eq(t._var("five"), t._false())),
    ]);
    let clause7 = Clause::new(vec![
        Literal::new(t._eq(t._var("two"), t._false())),
        Literal::new(t._eq(t._var("four"), t._false())),
    ]);
    let clause8 = Clause::new(vec![
        Literal::new(t._eq(t._var("two"), t._false())),
        Literal::new(t._eq(t._var("six"), t._false())),
    ]);
    let clause9 = Clause::new(vec![
        Literal::new(t._eq(t._var("four"), t._false())),
        Literal::new(t._eq(t._var("six"), t._false())),
    ]);
    let clauses = vec![
        clause1, clause2, clause3, clause4, clause5, clause6, clause7, clause8, clause9,
    ];
    let undecided = VecDeque::from(vec![
        t._var("one"),
        t._var("two"),
        t._var("three"),
        t._var("four"),
        t._var("five"),
        t._var("six"),
    ]);
    Solver::new(Box::new(t), clauses, undecided)
}

#[test]
fn test_sat_1() {
    assert_eq!(example_solver_sat_1().run(), true);
}

#[test]
fn test_sat_2() {
    assert_eq!(example_solver_sat_2().run(), true);
}

#[test]
fn test_unsat_1() {
    assert_eq!(example_solver_unsat_1().run(), false);
}

#[test]
fn test_unsat_2() {
    assert_eq!(example_solver_unsat_2().run(), false);
}

#[test]
fn test_unsat_pigeonhole_1() {
    assert_eq!(example_solver_pigeonhole_1().run(), false);
}

#[test]
fn test_unsat_pigeonhole_2() {
    assert_eq!(example_solver_pigeonhole_2().run(), false);
}
