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
use crate::theory::BooleanTheory;
use crate::types::variable::Variable;
use colored::*;

fn main() {
    // let clause1 = Clause::new(vec![Literal::new(
    //     less(variable("x"), variable("y")),
    //     false,
    // )]);
    // let clause2 = Clause::new(vec![Literal::new(
    //     less(variable("y"), variable("z")),
    //     false,
    // )]);
    // let clauses = vec![clause1, clause2];
    // let undecided = vec![Variable::new("x"), Variable::new("y"), Variable::new("z")];

    // let clause1 = Clause::new(vec![Literal::new(
    //     greater(
    //         variable("x"),
    //         variable("y"),
    //     ),
    //     false,
    // )]);
    // let clause2 = Clause::new(vec![Literal::new(
    //     greater(
    //         variable("x"),
    //         variable("y"),
    //     ),
    //     true,
    // )]);
    // let clauses = vec![clause1, clause2];
    // let undecided = vec![Variable::new("x"), Variable::new("y")];

    // let clause1 = Clause::new(vec![Literal::new(f(), false)]);
    // let clauses = vec![clause1];
    // let undecided = vec![];

    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t.var("a"), false, vec![Variable::new("a")]),
        Literal::new(t.var("b"), false, vec![Variable::new("b")]),
        Literal::new(t.var("c"), false, vec![Variable::new("c")]),
    ]);
    let clause2 = Clause::new(vec![Literal::new(
        t.var("a"),
        true,
        vec![Variable::new("a")],
    )]);

    let clauses = vec![clause1, clause2];
    let undecided = vec![Variable::new("a"), Variable::new("b"), Variable::new("c")];

    // let t = BooleanTheory::new();
    // let clause1 = Clause::new(vec![
    //     Literal::new(t.var("x"), false),
    //     Literal::new(t.var("y"), false),
    // ]);
    // let clause2 = Clause::new(vec![
    //     Literal::new(t.var("x"), false),
    //     Literal::new(t.var("y"), true),
    // ]);
    // let clause3 = Clause::new(vec![
    //     Literal::new(t.var("x"), true),
    //     Literal::new(t.var("y"), false),
    // ]);
    // let clause4 = Clause::new(vec![
    //     Literal::new(t.var("x"), true),
    //     Literal::new(t.var("y"), true),
    // ]);
    // let clauses = vec![clause1, clause2, clause3, clause4];
    // let undecided = vec![Variable::new("x"), Variable::new("y")];

    for c in &clauses {
        println!("{}", c);
    }
    println!("\n");

    let mut solver = Solver::new(Box::new(t), clauses, undecided);
    match solver.run() {
        true => println!("\n{}\n", "SAT".green()),
        false => println!("\n{}\n", "UNSAT".red()),
    }
}
