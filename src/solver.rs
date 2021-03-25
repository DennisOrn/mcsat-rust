use std::fmt;
use crate::model::Model;
use crate::state::State;
use crate::term::term::Term;
use crate::term::term::ActualTerm;
use crate::term::Value;
use crate::trail::Trail;
use crate::trail::TrailElement;

use crate::term::term::*;

#[derive(Debug)]
pub struct Solver {
    state: State,
    model: Model,
    trail: Trail,
    clauses: Vec<Term>,
    undecided: Vec<Term>,
}

impl Solver {
    pub fn new(clauses: Vec<Term>, undecided: Vec<Term>) -> Solver {
        Solver {
            state: State::Consistent,
            model: Model::new(),
            trail: Trail::new(),
            clauses: clauses,
            undecided: undecided
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            println!("\n{}", self);
            assert!(self.is_consistent());

            match &self.state {
                State::Consistent => {
                    if let Some(t) = self.undecided.pop() {

                        match t.get() {
                            ActualTerm::Boolean(_) => {
                                let new_value = Value::Bool(true);
                                self.model.set_value(t.clone(), new_value);
                                self.trail.push(TrailElement::DecidedLiteral(t.clone()));
                                // assert!(self.model.evaluate(&t.clone()) == Some(true));
                            }
                            // ActualTerm::Variable(_) => {
                            //     let new_value = Value::Integer(999);
                            //     self.model.set_value(t.clone(), new_value);
                            //     self.trail.push(TrailElement::ModelAssignment(t.clone(), new_value))
                            // }
                            _ => panic!()
                        }

                    } else {
                        assert!(self.is_complete());
                        return true
                    }

                }
                State::Conflict(conflict) => {
                    unimplemented!()
                }
            }
        }
    }

    fn is_consistent(&self) -> bool {
        println!("CHECK: is_consistent");
        for t in &self.clauses {
            match self.model.evaluate(&t) {
                Some(false) => { println!("{}: false", t); return false }
                Some(true) => { println!("{}: true", t); }
                None => { println!("{}: None", t); }
            }
        }
        true
    }

    fn is_complete(&self) -> bool {
        println!("CHECK: is_complete");
        for t in &self.clauses {
            match self.model.evaluate(&t) {
                Some(false) => { println!("{}: false", t); return false }
                Some(true) => { println!("{}: true", t); }
                None => { println!("{}: None", t); return false }
            }
        }
        true
    }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "SOLVER\nstate:\t\t{:?}\nmodel:\t\t{:?}\ntrail:\t\t{:?}\nclauses:\t{:?}\nundecided:\t{:?}",
            self.state, self.model, self.trail, self.clauses, self.undecided)
    }
}