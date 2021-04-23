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
fn test_evaluate_undefined_variable() {
    let model = Model::new();

    assert_eq!(
        less(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        None
    );
    assert_eq!(
        less_equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        None
    );
    assert_eq!(
        greater(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        None
    );
    assert_eq!(
        greater_equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        None
    );
    assert_eq!(
        equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        None
    );
}

#[test]
fn test_evaluate_defined_variable() {
    let mut model = Model::new();
    model.set_value(Variable::new("x"), Value::Integer(2));

    assert_eq!(
        less(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        Some(false)
    );
    assert_eq!(
        less_equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        Some(true)
    );
    assert_eq!(
        greater(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        Some(false)
    );
    assert_eq!(
        greater_equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        Some(true)
    );
    assert_eq!(
        equal(variable("x"), constant(Value::Integer(2))).evaluate(&model),
        Some(true)
    );
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

#[test]
fn test_trail_value_functions() {
    // EXAMPLE 1 FROM MCSAT-PAPER
    // M = [x > 1, x ↦ 1, y ↦ 0, z > 0]
    let mut trail = Trail::new();
    trail.push_decided_literal(Literal::new(
        greater(variable("x"), constant(Value::Integer(0))),
        false,
    ));
    trail.push_model_assignment(Variable::new("x"), Value::Integer(1));
    trail.push_model_assignment(Variable::new("y"), Value::Integer(0));
    trail.push_decided_literal(Literal::new(
        greater(variable("z"), constant(Value::Integer(0))),
        false,
    ));

    assert_eq!(
        trail.value_t(&Literal::new(
            greater(variable("x"), constant(Value::Integer(0))),
            false
        )),
        Some(true),
        "expected: value_t(x > 0) == true"
    );
    assert_eq!(
        trail.value_b(&Literal::new(
            greater(variable("x"), constant(Value::Integer(0))),
            false
        )),
        Some(true),
        "expected: value_b(x > 0) == true"
    );
    assert_eq!(
        trail.value_t(&Literal::new(
            greater(variable("x"), constant(Value::Integer(1))),
            false
        )),
        Some(false),
        "expected: value_t(x > 1) == false"
    );
    assert_eq!(
        trail.value_t(&Literal::new(
            greater(variable("z"), constant(Value::Integer(0))),
            false
        )),
        None,
        "expected: value_t(z > 0) == None"
    );
    assert_eq!(
        trail.value_b(&Literal::new(
            greater(variable("z"), constant(Value::Integer(0))),
            false
        )),
        Some(true),
        "expected: value_b(z > 0) == true"
    );
}
