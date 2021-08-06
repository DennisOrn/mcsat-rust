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
use crate::term::term::Term;
use crate::theory::BooleanTheory;
use colored::*;
use hashconsing::HConsed;
use std::collections::VecDeque;

use env_logger::Builder;
use log::{debug, error, info, trace, warn, LevelFilter};
use std::io::Write;

fn main() {
    // env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, LevelFilter::Trace)
        .init();

    error!("this is error log");
    warn!("this is warning log");
    info!("this is information log");
    debug!("this is debug log");
    trace!("this is trace log");

    example_unsat()
    // for _ in 0..10000 {
    //     example_unsat();
    // }
}

fn example_unsat() {
    let t = BooleanTheory::new();
    let clause1 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._true())),
    ]);
    let clause2 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._true())),
        Literal::new(t._eq(t._var("y"), t._false())),
    ]);
    let clause3 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._true())),
    ]);
    let clause4 = Clause::new(vec![
        Literal::new(t._eq(t._var("x"), t._false())),
        Literal::new(t._eq(t._var("y"), t._false())),
    ]);

    let clauses = vec![clause1, clause2, clause3, clause4];
    let undecided: VecDeque<HConsed<Term>> = VecDeque::from(vec![t._var("x"), t._var("y")]);
    let mut solver = Solver::new(Box::new(t), clauses, undecided);

    // match solver.run() {
    match solver.run_hardcoded() {
        true => println!("{}", "SAT".green()),
        false => println!("{}", "UNSAT".red()),
        _ => (),
    }
}
