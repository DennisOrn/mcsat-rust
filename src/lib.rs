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
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
    ]);

    let clauses = vec![clause1, clause2, clause3, clause4];
    let undecided = VecDeque::from(vec![t.Var("x"), t.Var("y")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_unsat_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
        Literal::new(t.Eq(t.Var("z"), t.True())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
        Literal::new(t.Eq(t.Var("z"), t.False())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
        Literal::new(t.Eq(t.Var("z"), t.True())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
        Literal::new(t.Eq(t.Var("z"), t.False())),
    ]);
    let clause5 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
        Literal::new(t.Eq(t.Var("z"), t.True())),
    ]);
    let clause6 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
        Literal::new(t.Eq(t.Var("z"), t.False())),
    ]);
    let clause7 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
        Literal::new(t.Eq(t.Var("z"), t.True())),
    ]);
    let clause8 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
        Literal::new(t.Eq(t.Var("z"), t.False())),
    ]);

    let clauses = vec![
        clause1, clause2, clause3, clause4, clause5, clause6, clause7, clause8,
    ];
    let undecided = VecDeque::from(vec![t.Var("x"), t.Var("y"), t.Var("z")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_sat_1() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.True())),
        Literal::new(t.Eq(t.Var("y"), t.True())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("x"), t.False())),
        Literal::new(t.Eq(t.Var("y"), t.False())),
    ]);

    let clauses = vec![clause1, clause2];
    let undecided = VecDeque::from(vec![t.Var("x"), t.Var("y")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_sat_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![Literal::new(t.Eq(t.Var("x"), t.True()))]);

    let clauses = vec![clause1];
    let undecided = VecDeque::from(vec![t.Var("x")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_pigeonhole_1() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![Literal::new(t.Eq(t.Var("one"), t.True()))]);
    let clause2 = Clause::new(vec![Literal::new(t.Eq(t.Var("two"), t.True()))]);
    let clause3 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("one"), t.False())),
        Literal::new(t.Eq(t.Var("two"), t.False())),
    ]);
    let clauses = vec![clause1, clause2, clause3];
    let undecided = VecDeque::from(vec![t.Var("one"), t.Var("two")]);
    Solver::new(Box::new(t), clauses, undecided)
}

pub fn example_solver_pigeonhole_2() -> Solver {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("one"), t.True())),
        Literal::new(t.Eq(t.Var("two"), t.True())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("three"), t.True())),
        Literal::new(t.Eq(t.Var("four"), t.True())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("five"), t.True())),
        Literal::new(t.Eq(t.Var("six"), t.True())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("one"), t.False())),
        Literal::new(t.Eq(t.Var("three"), t.False())),
    ]);
    let clause5 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("one"), t.False())),
        Literal::new(t.Eq(t.Var("five"), t.False())),
    ]);
    let clause6 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("three"), t.False())),
        Literal::new(t.Eq(t.Var("five"), t.False())),
    ]);
    let clause7 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("two"), t.False())),
        Literal::new(t.Eq(t.Var("four"), t.False())),
    ]);
    let clause8 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("two"), t.False())),
        Literal::new(t.Eq(t.Var("six"), t.False())),
    ]);
    let clause9 = Clause::new(vec![
        Literal::new(t.Eq(t.Var("four"), t.False())),
        Literal::new(t.Eq(t.Var("six"), t.False())),
    ]);
    let clauses = vec![
        clause1, clause2, clause3, clause4, clause5, clause6, clause7, clause8, clause9,
    ];
    let undecided = VecDeque::from(vec![
        t.Var("one"),
        t.Var("two"),
        t.Var("three"),
        t.Var("four"),
        t.Var("five"),
        t.Var("six"),
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
