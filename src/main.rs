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
use crate::trail::Trail;
use crate::trail::TrailElement;

fn main() {
    // x < 1 ∨ p, ¬p ∨ x = 2
    let clause1 = Clause::new(vec![
        predicate(LessThan, vec![variable("x"), constant(Value::Integer(1))]),
        predicate(Equal, vec![variable("p"), constant(Value::Bool(true))]),
    ]);
    let clause2 = Clause::new(vec![
        predicate(Equal, vec![variable("p"), constant(Value::Bool(false))]),
        predicate(Equal, vec![variable("x"), constant(Value::Integer(2))]),
    ]);

    let clauses = vec![clause1, clause2];
    let undecided = vec![variable("x")];

    for c in &clauses {
        println!("{}", c);
    }

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT"),
        false => println!("\nUNSAT"),
    }

    let mut trail = Trail::new();

    let f1 = predicate(Equal, vec![variable("x"), constant(Value::Integer(2))]);
    let decided_literal = TrailElement::DecidedLiteral(f1.get().clone());
    trail.push(decided_literal);

    let c2 = Clause::new(vec![predicate(
        Equal,
        vec![variable("p"), constant(Value::Bool(false))],
    )]);

    let f2 = predicate(LessThan, vec![variable("x"), constant(Value::Integer(1))]);
    let propagated_literal = TrailElement::PropagatedLiteral(c2, f2.get().clone());
    trail.push(propagated_literal);

    match trail.value_b(f2) {
        Some(true) => println!("True"),
        Some(false) => println!("False"),
        None => println!("None"),
    };
}
