use std::fmt;
use crate::model::Model;
use crate::state::State;
use crate::term::term::Term;
use crate::term::term::ActualTerm;
use crate::term::Value;
use crate::trail::Trail;
use crate::trail::TrailElement;

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
            println!("{}", self);

            match &self.state {
                State::Consistent => {
                    if let Some(t) = self.undecided.pop() {

                        match t.get() {
                            ActualTerm::Literal(_) => {
                                let new_value = Value::Bool(true);
                                self.model.set_value(t.clone(), new_value);
                                self.trail.push(TrailElement::DecidedLiteral(t))
                            }
                            ActualTerm::Variable(_) => {
                                let new_value = Value::Integer(999);
                                self.model.set_value(t.clone(), new_value);
                                self.trail.push(TrailElement::ModelAssignment(t, new_value))
                            }
                            _ => panic!()
                        }

                    } else {
                        return true
                    }

                }
                State::Conflict(conflict) => {
                    unimplemented!()
                }
            }
        }
    }

    // fn decide(&mut self, id: i32) {
    //     let trail_element = Variable::new(id, Boolean::True);
    //     self.trail.push(trail_element);
    // }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "SOLVER\nstate:\t\t{:?}\nmodel:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}\n",
            self.state, self.model, self.trail, self.undecided)
    }
}