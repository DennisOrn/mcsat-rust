mod model;
mod solver;
mod state;
mod term;
mod trail;

use crate::solver::Solver;
use crate::term::term::*;

fn main() {

    // x < 1 ∨ p, ¬p ∨ x = 2
    // let t1 = disjunction(less_than(variable("x"), constant(1)), literal("p"));
    // let t2 = disjunction(negation(literal("p")), equal(variable("x"), constant(2)));
    // println!("{}", t1);
    // println!("{}\n", t2);

    let e1 = disjunction(disjunction(boolean("x1"), boolean("x2")), boolean("x3"));
    let e2 = disjunction(negation(boolean("x1")), disjunction(boolean("x2"), f()));

    let expressions: Vec<Expr> = vec![e1, e2];
    let undecided: Vec<Expr> = vec![boolean("x1"), boolean("x2"), boolean("x3")];

    println!("Clauses:");
    for e in &expressions {
        println!("{}", e)
    }

    let mut solver = Solver::new(expressions, undecided);
    match solver.run() {
        true => println!("\nSAT"),
        false => println!("\nUNSAT")
    }
}