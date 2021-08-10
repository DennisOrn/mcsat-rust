use mcsat_rust::configure_logger;
use mcsat_rust::example_solver_unsat;

use colored::*;
use log::LevelFilter;

pub fn main() {
    configure_logger(LevelFilter::Debug);
    let mut solver = example_solver_unsat();
    match solver.run() {
        true => println!("{}", "SAT".green()),
        false => println!("{}", "UNSAT".red()),
    }
}
