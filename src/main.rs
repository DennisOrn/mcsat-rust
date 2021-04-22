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

    let clause1 = Clause::new(vec![Literal::new(
        less(variable("x"), constant(Value::Integer(1))),
        false,
    )]);

    let clause2 = Clause::new(vec![Literal::new(
        equal(variable("x"), constant(Value::Integer(2))),
        true,
    )]);

    let clauses = vec![clause1, clause2];
    let undecided = vec![variable("x"), variable("p")];

    for c in &clauses {
        println!("{}", c);
    }

    let mut solver = Solver::new(clauses, undecided);
    match solver.run() {
        true => println!("\nSAT\n"),
        false => println!("\nUNSAT\n"),
    }

    // Evaluate test
    let mut model = Model::new();
    assert!(
        less(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        less_equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        greater(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        greater_equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == None
    );

    model.set_value(Variable::new("x"), Value::Integer(2));

    assert!(
        less(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == Some(false)
    );
    assert!(
        less_equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == Some(true)
    );
    assert!(
        greater(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == Some(false)
    );
    assert!(
        greater_equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == Some(true)
    );
    assert!(
        equal(variable("x"), constant(Value::Integer(2)))
            .get()
            .evaluate(&model)
            == Some(true)
    );

    assert!(
        equal(
            variable("x"),
            plus(constant(Value::Integer(1)), constant(Value::Integer(1)))
        )
        .get()
        .evaluate(&model)
            == Some(true)
    );
    assert!(
        equal(
            variable("x"),
            plus(constant(Value::Integer(1)), constant(Value::Integer(2)))
        )
        .get()
        .evaluate(&model)
            == Some(false)
    );
    assert!(
        equal(
            plus(constant(Value::Integer(1)), constant(Value::Integer(6))),
            minus(constant(Value::Integer(10)), constant(Value::Integer(3)))
        )
        .get()
        .evaluate(&model)
            == Some(true)
    );

    // Trail test
    // EXAMPLE 1 FROM MCSAT-PAPER
    // M = [[x > 1, x ↦ 1, y ↦ 0, z > 0]]
    println!("\nTRAIL TEST\n");
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

    assert!(
        trail.value_t(&Literal::new(
            greater(variable("x"), constant(Value::Integer(0))),
            false
        )) == Some(true),
        "expected: value_t(x > 0) == true"
    );
    assert!(
        trail.value_b(&Literal::new(
            greater(variable("x"), constant(Value::Integer(0))),
            false
        )) == Some(true),
        "expected: value_b(x > 0) == true"
    );
    assert!(
        trail.value_t(&Literal::new(
            greater(variable("x"), constant(Value::Integer(1))),
            false
        )) == Some(false),
        "expected: value_t(x > 1) == false"
    );
    assert!(
        trail.value_t(&Literal::new(
            greater(variable("z"), constant(Value::Integer(0))),
            false
        )) == None,
        "expected: value_t(z > 0) == None"
    );
    assert!(
        trail.value_b(&Literal::new(
            greater(variable("z"), constant(Value::Integer(0))),
            false
        )) == Some(true),
        "expected: value_b(z > 0) == true"
    );
}
