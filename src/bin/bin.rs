use mcsat_rust::configure_logger;

use colored::*;
use log::LevelFilter;

pub fn main() {
    configure_logger(LevelFilter::Debug);
    let mut solver = mcsat_rust::example_solver_unsat_1();
    match solver.run() {
        true => println!("{}", "SAT".green()),
        false => println!("{}", "UNSAT".red()),
    }
}
