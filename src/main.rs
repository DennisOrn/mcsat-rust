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

    let t1 = disjunction(disjunction(literal("x1"), literal("x2")), literal("x3"));
    let t2 = disjunction(negation(literal("x1")), literal("x2"));

    let terms: Vec<Term> = vec![t1, t2];
    let undecided: Vec<Term> = vec![literal("x1"), literal("x2"), literal("x3")];

    for term in &terms {
        println!("{}", term)
    }

    let mut solver = Solver::new(terms, undecided);
    match solver.run() {
        true => println!("SAT"),
        false => println!("UNSAT")
    }
}