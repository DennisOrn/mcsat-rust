mod clause;
mod formula;
mod model;
mod solver;
mod state;
mod term;
mod trail;

use crate::clause::Clause;
use crate::formula::formula::Predicate::*;
use crate::formula::formula::*;
use crate::solver::Solver;
use crate::term::term::*;

fn main() {
    // x < 1 ∨ p, ¬p ∨ x = 2
    // let e1 = disjunction(disjunction(boolean("x1"), boolean("x2")), boolean("x3"));
    // let e2 = disjunction(negation(boolean("x1")), disjunction(boolean("x2"), f()));

    // let expressions: Vec<Expr> = vec![e1, e2];
    // let undecided: Vec<Expr> = vec![boolean("x1"), boolean("x2"), boolean("x3")];

    // println!("Clauses:");
    // for e in &expressions {
    //     println!("{}", e)
    // }

    // let mut solver = Solver::new(expressions, undecided);
    // match solver.run() {
    //     true => println!("\nSAT"),
    //     false => println!("\nUNSAT")
    // }


    let formula1 = predicate(LessThanOrEqual, vec![variable("x"), constant(15)]);
    let formula2 = predicate(Equal, vec![variable("x"), constant(15)]);

    println!("{}", formula1);
    println!("{}", formula2);

    let clause = Clause::new(vec![formula1, formula2]);
    let clauses = vec![clause];
    let undecided = vec![variable("x")];

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT"),
        false => println!("\nUNSAT")
    }
}
