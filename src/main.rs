mod model;
mod solver;
mod state;
mod term;
mod trail;

use crate::solver::Solver;
use crate::term::term::*;

fn main() {

    // x < 1 ∨ p, ¬p ∨ x = 2
    let t1 = disjunction(less_than(variable("x"), constant(1)),literal("p"));
    let t2 = disjunction(negation(literal("p")), equal(variable("x"), constant(2)));

    println!("{}", t1);
    println!("{}\n", t2);

    let mut solver = Solver::new(vec![t1, t2],
                                 vec![variable("x"), literal("p")]);
    match solver.run() {
        true => println!("SAT"),
        false => println!("UNSAT")
    }
}