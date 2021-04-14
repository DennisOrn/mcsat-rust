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

fn main() {
    // EXAMPLE 2 FROM MCSAT-PAPER
    // x < 1 ∨ p, ¬p ∨ x = 2
    let clause1 = Clause::new(vec![
        predicate(LessThan, vec![variable("x"), constant(Value::Integer(1))]),
        predicate(Equal, vec![variable("p"), constant(Value::Boolean(true))]),
    ]);
    let clause2 = Clause::new(vec![
        predicate(Equal, vec![variable("p"), constant(Value::Boolean(false))]),
        predicate(Equal, vec![variable("x"), constant(Value::Integer(2))]),
    ]);

    let clauses = vec![clause1, clause2];
    let undecided = vec![variable("x"), variable("p")];

    for c in &clauses {
        println!("{}", c);
    }

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT"),
        false => println!("\nUNSAT"),
    }

    // Trail test
    // EXAMPLE 1 FROM MCSAT-PAPER
    // M = [[x > 1, x ↦ 1, y ↦ 0, z > 0]]
    let mut trail = Trail::new();

    trail.push_decided_literal(predicate(
        GreaterThan,
        vec![variable("x"), constant(Value::Integer(0))],
    ));
    trail.push_model_assignment(Variable::new("x"), Value::Integer(1));
    trail.push_model_assignment(Variable::new("y"), Value::Integer(0));
    trail.push_decided_literal(predicate(
        GreaterThan,
        vec![variable("z"), constant(Value::Integer(0))],
    ));

    assert!(
        trail.value_t(&predicate(
            GreaterThan,
            vec![variable("x"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_t(x > 0) == true"
    );
    assert!(
        trail.value_b(&predicate(
            GreaterThan,
            vec![variable("x"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_b(x > 0) == true"
    );
    assert!(
        trail.value_t(&predicate(
            GreaterThan,
            vec![variable("x"), constant(Value::Integer(1))],
        )) == Some(false),
        "expected: value_t(x > 1) == false"
    );
    assert!(
        trail.value_t(&predicate(
            GreaterThan,
            vec![variable("z"), constant(Value::Integer(0))],
        )) == None,
        "expected: value_t(z > 0) == None"
    );
    assert!(
        trail.value_b(&predicate(
            GreaterThan,
            vec![variable("z"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_b(z > 0) == true"
    );
}
