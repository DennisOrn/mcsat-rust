mod experimental;
mod model;
mod solver;
mod state;
mod term;
mod trail;

// use crate::experimental::experimental;
use crate::solver::Solver;
use crate::model::Model;
// use crate::term::term::*;
use std::collections::HashMap;

fn main() {

    // experimental::experimental::foo();

    let x1 = experimental::experimental::boolean("x1");
    let x2 = experimental::experimental::boolean("x2");
    let or1 = experimental::experimental::or(x1.get(), x2.get());

    let model = Model::new();
    let result = model.evaluate_expression(&or1);
    println!("{:?}", result);
    // model.evaluate_expression::<experimental::experimental::Boolean>(b1.get()); // same thing^


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