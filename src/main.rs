mod clause;
mod formula;
mod literal;
mod model;
mod solver;
mod state;
mod term;
mod theory;
mod trail;
mod trail_element; // TODO: underscore?
mod types;

use crate::clause::Clause;
use crate::literal::Literal;
use crate::solver::Solver;
use crate::term::term::variable;
use crate::theory::BooleanTheory;
use crate::types::variable::Variable;
use colored::*;
use std::collections::HashMap;

fn main() {
    // let clause1 = Clause::new(vec![Literal::new(
    //     less(variable("x"), variable("y")),
    //     false,
    // )]);
    // let clause2 = Clause::new(vec![Literal::new(
    //     less(variable("y"), variable("z")),
    //     false,
    // )]);
    // let clauses = vec![clause1, clause2];
    // let undecided = vec![Variable::new("x"), Variable::new("y"), Variable::new("z")];

    // let clause1 = Clause::new(vec![Literal::new(
    //     greater(
    //         variable("x"),
    //         variable("y"),
    //     ),
    //     false,
    // )]);
    // let clause2 = Clause::new(vec![Literal::new(
    //     greater(
    //         variable("x"),
    //         variable("y"),
    //     ),
    //     true,
    // )]);
    // let clauses = vec![clause1, clause2];
    // let undecided = vec![Variable::new("x"), Variable::new("y")];

    // let clause1 = Clause::new(vec![Literal::new(f(), false)]);
    // let clauses = vec![clause1];
    // let undecided = vec![];

    // let t = BooleanTheory::new();

    // let literal1 = Literal::new(t.var("a"), false, vec![Variable::new("a")]);
    // let literal2 = Literal::new(t.var("b"), false, vec![Variable::new("b")]);
    // let literal3 = Literal::new(t.var("c"), false, vec![Variable::new("c")]);
    // let literal4 = Literal::new(t.var("a"), true, vec![Variable::new("a")]);

    // // let mut map: HashMap<Variable, Vec<&Literal>> = HashMap::new();
    // // map.insert(Variable::new("a"), vec![&literal1, &literal4]);
    // // map.insert(Variable::new("b"), vec![&literal2]);
    // // map.insert(Variable::new("c"), vec![&literal3]);

    // let clause1 = Clause::new(vec![literal1, literal2, literal3]);
    // let clause2 = Clause::new(vec![literal4]);

    // let mut map: HashMap<Variable, Vec<&Clause>> = HashMap::new();
    // map.insert(Variable::new("a"), vec![&clause1, &clause2]);
    // map.insert(Variable::new("b"), vec![&clause1]);
    // map.insert(Variable::new("c"), vec![&clause1]);

    // let clauses = vec![clause1, clause2];
    // // let clauses = map.values().cloned().collect::<Vec<&Clause>>();
    // let undecided = vec![Variable::new("a"), Variable::new("b"), Variable::new("c")];

    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t.var("x"), vec![variable("x")], false),
        Literal::new(t.var("y"), vec![variable("y")], false),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t.var("x"), vec![variable("x")], false),
        Literal::new(t.var("y"), vec![variable("y")], true),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t.var("x"), vec![variable("x")], true),
        Literal::new(t.var("y"), vec![variable("y")], false),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t.var("x"), vec![variable("x")], true),
        Literal::new(t.var("y"), vec![variable("y")], true),
    ]);
    let clauses = vec![clause1, clause2, clause3, clause4];
    let undecided = vec![variable("x"), variable("y")];

    for c in &clauses {
        println!("{}", c);
    }
    println!("\n");

    let mut solver = Solver::new(Box::new(t), clauses, undecided /*, map*/);
    // match solver.run_hardcoded_example() {
    match solver.run() {
        true => println!("\n{}\n", "SAT".green()),
        false => println!("\n{}\n", "UNSAT".red()),
    }
}
