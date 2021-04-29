mod clause;
mod formula;
mod literal;
mod model;
mod solver;
mod state;
mod term;
mod trail;
mod trail_element; // TODO: underscore?
mod types;

use crate::clause::Clause;
use crate::formula::formula::{equal, f, greater, greater_equal, less, less_equal, t};
use crate::literal::Literal;
use crate::solver::Solver;
use crate::term::term::{constant, minus, plus, variable};
use crate::types::value::Value;
use crate::types::variable::Variable;

fn main() {
    let clause1 = Clause::new(vec![Literal::new(
        less(variable("x"), variable("y")),
        false,
    )]);
    let clause2 = Clause::new(vec![Literal::new(
        less(variable("y"), variable("z")),
        false,
    )]);
    let clauses = vec![clause1, clause2];
    let undecided = vec![Variable::new("x"), Variable::new("y"), Variable::new("z")];

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

    for c in &clauses {
        println!("{}", c);
    }
    println!("\n\n");

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT\n"),
        false => println!("\nUNSAT\n"),
    }
}
