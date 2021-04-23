mod clause;
mod formula;
mod literal;
mod model;
mod solver;
mod state;
mod term;
mod trail;
mod types;

use crate::clause::Clause;
use crate::formula::formula::{equal, greater, greater_equal, less, less_equal};
use crate::literal::Literal;
use crate::model::Model;
use crate::solver::Solver;
use crate::term::term::{constant, minus, plus, variable};
use crate::trail::Trail;
use crate::types::value::Value;
use crate::types::variable::Variable;

fn main() {
    // EXAMPLE 2 FROM MCSAT-PAPER
    // x < 1 ∨ p, ¬p ∨ x = 2
    // let clause1 = Clause::new(vec![
    //     predicate(Less, vec![variable("x"), constant(Value::Integer(1))]),
    //     predicate(Equal, vec![variable("p"), constant(Value::Boolean(true))]),
    // ]);
    // let clause2 = Clause::new(vec![
    //     predicate(Equal, vec![variable("p"), constant(Value::Boolean(false))]),
    //     predicate(Equal, vec![variable("x"), constant(Value::Integer(2))]),
    // ]);

    // let clause1 = Clause::new(vec![Literal::new(
    //     less(variable("x"), constant(Value::Integer(1))),
    //     false,
    // )]);

    // let clause2 = Clause::new(vec![Literal::new(
    //     equal(variable("x"), constant(Value::Integer(2))),
    //     true,
    // )]);

    // let clauses = vec![clause1, clause2];
    // let undecided = vec![variable("x"), variable("p")];

    // for c in &clauses {
    //     println!("{}", c);
    // }

    // let mut solver = Solver::new(clauses, undecided);
    // match solver.run() {
    //     true => println!("\nSAT\n"),
    //     false => println!("\nUNSAT\n"),
    // }
}

#[test]
fn test_evaluate_literal() {
    let model = Model::new();

    assert_eq!(
        Literal::new(
            equal(
                plus(constant(Value::Integer(1)), constant(Value::Integer(1))),
                constant(Value::Integer(2))
            ),
            false
        )
        .evaluate(&model),
        Some(true)
    );
    assert_eq!(
        Literal::new(
            equal(
                plus(constant(Value::Integer(1)), constant(Value::Integer(1))),
                constant(Value::Integer(2))
            ),
            true
        )
        .evaluate(&model),
        Some(false)
    );
    assert_eq!(
        Literal::new(
            equal(
                plus(constant(Value::Integer(1)), constant(Value::Integer(1))),
                variable("x")
            ),
            false
        )
        .evaluate(&model),
        None
    );
}

#[test]
fn test_evaluate_function_undefined_variable() {
    let model = Model::new();

    assert_eq!(
        equal(
            variable("x"),
            plus(constant(Value::Integer(1)), constant(Value::Integer(1)))
        )
        .evaluate(&model),
        None
    );
}

#[test]
fn test_evaluate_function_defined_variable() {
    let mut model = Model::new();
    model.set_value(Variable::new("x"), Value::Integer(2));

    assert_eq!(
        equal(
            variable("x"),
            plus(constant(Value::Integer(1)), constant(Value::Integer(1)))
        )
        .evaluate(&model),
        Some(true)
    );
    assert_eq!(
        equal(
            variable("x"),
            plus(constant(Value::Integer(1)), constant(Value::Integer(2)))
        )
        .evaluate(&model),
        Some(false)
    );
    assert_eq!(
        equal(
            plus(constant(Value::Integer(1)), constant(Value::Integer(6))),
            minus(constant(Value::Integer(9)), variable("x"))
        )
        .evaluate(&model),
        Some(true)
    );
}
