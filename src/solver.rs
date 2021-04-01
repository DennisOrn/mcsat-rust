use std::fmt;
use crate::model::Model;
use crate::state::State;
use crate::term::term::Expr;
use crate::term::term::Expression;
use crate::term::term::Term;
use crate::term::Value;
use crate::trail::Trail;
use crate::trail::TrailElement;


pub struct Solver {
    state: State,
    model: Model,
    trail: Trail,
    clauses: Vec<Expr>,
    undecided: Vec<Expr>,
}

impl Solver {
    pub fn new(clauses: Vec<Expr>, undecided: Vec<Expr>) -> Solver {
        Solver {
            state: State::Search,
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



            // while !self.is_consistent() {
            //     println!("NOT CONSISTENT");
            //     let trail_element = self.trail.pop().unwrap();
            //     println!("POP TRAIL ELEMENT: {:?}", trail_element);
            //     match trail_element {
            //         TrailElement::DecidedLiteral(t) => {
            //             self.model.clear_value(t.clone());
            //             self.undecided.push(t.clone());
            //             self.state = State::Conflict(t.clone());
            //             println!("\n{}", self)
            //         }
            //         _ => panic!()
            //     }
            // }



            match &self.state {
                State::Search => {
                    if let Some(e) = self.undecided.pop() {

                        match e.get() {
                            Expression::Term(t) => {

                                match t {
                                    Term::Boolean(_) => {

                                        let new_value = Value::Bool(true);
                                        self.model.set_value(t.clone(), new_value);
                                        self.trail.push(TrailElement::DecidedLiteral(t.clone()));
                                        // assert!(self.model.evaluate(&t.clone()) == Some(true));
                                    }
                                    _ => panic!()
                                }
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
                State::Conflict(_) => {
                    unimplemented!()
                }
            }
        }
    }




    fn is_consistent(&self) -> bool { // TODO: trail
        println!("\nCHECK: is_consistent");
        for e in &self.clauses {
            match self.model.evaluate(&e) {
                Some(false) => { println!("{}: false", e); return false }
                Some(true) => { println!("{}: true", e); }
                None => { println!("{}: None", e); }
            }
        }
        true
    }

    fn is_complete(&self) -> bool {
        println!("\nCHECK: is_complete");
        for e in &self.clauses {
            match self.model.evaluate(&e) {
                Some(false) => { println!("{}: false", e); return false }
                Some(true) => { println!("{}: true", e); }
                None => { println!("{}: None", e); return false }
            }
        }
        true
    }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            // "SOLVER\nstate:\t\t{:?}\nmodel:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}",
            // self.state, self.model, self.trail, self.undecided)
            "SOLVER\nstate:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}",
            self.state, self.trail, self.undecided)
    }
}