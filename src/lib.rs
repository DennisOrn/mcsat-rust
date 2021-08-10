pub mod clause;
mod formula;
pub mod literal;
mod model;
pub mod solver;
mod state;
mod term;
pub mod theory;
mod trail;
mod trail_element;
mod types;

use crate::term::term::Term;
use crate::theory::BooleanTheory;
use crate::types::variable::Variable;

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/**
Set the log level to adjust tha amount of information that is printed.
From low to high priority: Trace / Debug / Info / Warn / Error
*/
pub fn configure_logger(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, level)
        .init();
}
