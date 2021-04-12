mod types;
mod experimental;
mod model;
mod solver;
mod state;
mod term;
mod trail;

// use crate::experimental::experimental;
use crate::solver::Solver;
use crate::model::Model;
use crate::experimental::experimental::Value;
// use crate::term::term::*;
use types::*;
use std::collections::HashMap;

fn main() {

    // let x1 = boolean::boolean("x1");
    // let x2 = boolean::boolean("x2");
    // let or = or::or(&x1, &x2);

    let mut model = Model::new();

    let x1 = boolean::new("x1");
    model.set_boolean(x1.clone(), Value::Bool(true));

    let x2 = boolean::new("x2");
    let x3 = boolean::new("x3");
    let or = or::new(vec![Box::new(x1), Box::new(x2), Box::new(x3)]);

    match model.evaluate(&or) {
        Some(true)  => println!("True"),
        Some(false) => println!("False"),
        None        => println!("None")
    }


    // model.evaluate_expression(b1.get());

    // let expression = Boolean { id: "x1".to_string() } as Expression;
    // model.evaluate_expression(e);



    // let foo = boolean("x", Some(true));
    // foo.evaluate();

    // let bar = or(vec![negation(boolean("x1", Some(true))), boolean("x2", Some(false))]);


    // x < 1 ∨ p, ¬p ∨ x = 2
    // let t1 = disjunction(less_than(variable("x"), constant(1)), literal("p"));
    // let t2 = disjunction(negation(literal("p")), equal(variable("x"), constant(2)));
    // println!("{}", t1);
    // println!("{}\n", t2);

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
}