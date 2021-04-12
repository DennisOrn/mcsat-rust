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
    let clause1 = Clause::new(vec![
        predicate(LessThanOrEqual, vec![variable("x"), constant(Value::Integer(1))]),
        predicate(Equal, vec![variable("p"), constant(Value::Bool(true))]),
    ]);
    let clause2 = Clause::new(vec![
        predicate(Equal, vec![variable("p"), constant(Value::Bool(false))]),
        predicate(Equal, vec![variable("x"), constant(Value::Integer(2))]),
    ]);

    let clauses = vec![clause1, clause2];
    let undecided = vec![variable("x")];

    for x in &clauses {
        println!("{}", x);
    }

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT"),
        false => println!("\nUNSAT")
    }
}
