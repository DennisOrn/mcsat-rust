mod clause;
mod formula;
mod model;
mod solver;
mod state;
mod term;
mod trail;
mod types;

use crate::clause::Clause;
use crate::formula::formula::*;
use crate::solver::Solver;
use crate::term::term::*;
use crate::trail::Trail;
use crate::types::function::Function::*;
use crate::types::predicate::Predicate::*;
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

    // Evaluate test
    let mut model = model::Model::new();
    assert!(
        predicate(Less, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        predicate(LessEqual, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        predicate(Greater, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == None
    );
    assert!(
        predicate(
            GreaterEqual,
            vec![variable("x"), constant(Value::Integer(2))]
        )
        .get()
        .evaluate(&model)
            == None
    );
    assert!(
        predicate(Equal, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == None
    );

    model.set_value(Variable::new("x"), Value::Integer(2));

    assert!(
        predicate(Less, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == Some(false)
    );
    assert!(
        predicate(LessEqual, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == Some(true)
    );
    assert!(
        predicate(Greater, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == Some(false)
    );
    assert!(
        predicate(
            GreaterEqual,
            vec![variable("x"), constant(Value::Integer(2))]
        )
        .get()
        .evaluate(&model)
            == Some(true)
    );
    assert!(
        predicate(Equal, vec![variable("x"), constant(Value::Integer(2))])
            .get()
            .evaluate(&model)
            == Some(true)
    );

    assert!(
        predicate(
            Equal,
            vec![
                variable("x"),
                function(
                    Plus,
                    vec![constant(Value::Integer(1)), constant(Value::Integer(1))]
                )
            ]
        )
        .get()
        .evaluate(&model)
            == Some(true)
    );
    assert!(
        predicate(
            Equal,
            vec![
                variable("x"),
                function(
                    Plus,
                    vec![constant(Value::Integer(1)), constant(Value::Integer(2))]
                )
            ]
        )
        .get()
        .evaluate(&model)
            == Some(false)
    );
    assert!(
        predicate(
            Equal,
            vec![
                function(
                    Plus,
                    vec![constant(Value::Integer(1)), constant(Value::Integer(6))]
                ),
                function(
                    Minus,
                    vec![constant(Value::Integer(10)), constant(Value::Integer(3))]
                )
            ]
        )
        .get()
        .evaluate(&model)
            == Some(true)
    );

    // Trail test
    // EXAMPLE 1 FROM MCSAT-PAPER
    // M = [[x > 1, x ↦ 1, y ↦ 0, z > 0]]
    let mut trail = Trail::new();

    trail.push_decided_literal(predicate(
        Greater,
        vec![variable("x"), constant(Value::Integer(0))],
    ));
    trail.push_model_assignment(Variable::new("x"), Value::Integer(1));
    trail.push_model_assignment(Variable::new("y"), Value::Integer(0));
    trail.push_decided_literal(predicate(
        Greater,
        vec![variable("z"), constant(Value::Integer(0))],
    ));

    assert!(
        trail.value_t(&predicate(
            Greater,
            vec![variable("x"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_t(x > 0) == true"
    );
    assert!(
        trail.value_b(&predicate(
            Greater,
            vec![variable("x"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_b(x > 0) == true"
    );
    assert!(
        trail.value_t(&predicate(
            Greater,
            vec![variable("x"), constant(Value::Integer(1))],
        )) == Some(false),
        "expected: value_t(x > 1) == false"
    );
    assert!(
        trail.value_t(&predicate(
            Greater,
            vec![variable("z"), constant(Value::Integer(0))],
        )) == None,
        "expected: value_t(z > 0) == None"
    );
    assert!(
        trail.value_b(&predicate(
            Greater,
            vec![variable("z"), constant(Value::Integer(0))],
        )) == Some(true),
        "expected: value_b(z > 0) == true"
    );
}
