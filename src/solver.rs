use crate::clause::Clause;
use crate::literal::Literal;
use crate::state::State;
use crate::term::term::Term;
use crate::trail::Trail;
use crate::types::variable::Variable;
use hashconsing::HConsed;

#[derive(Debug)]
pub struct Solver {
    state: State,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: Vec<Variable>,
}

impl Solver {
    pub fn new(clauses: Vec<Clause>, undecided: Vec<Variable>) -> Solver {
        Solver {
            state: State::Search,
            trail: Trail::new(),
            clauses: clauses,
            undecided: undecided,
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            println!("\n{}", self);
            return false; // TODO: REMOVE THIS

            // assert!(self.is_consistent());

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

            // match &self.state {
            //     State::Search => {
            //         if let Some(e) = self.undecided.pop() {

            //             match e.get() {
            //                 Expression::Term(t) => {

            //                     match t {
            //                         Term::Boolean(_) => {

            //                             let new_value = Value::Bool(true);
            //                             self.model.set_value(t.clone(), new_value);
            //                             self.trail.push(TrailElement::DecidedLiteral(t.clone()));
            //                             // assert!(self.model.evaluate(&t.clone()) == Some(true));
            //                         }
            //                         _ => panic!()
            //                     }
            //                 }

            //                 // ActualTerm::Variable(_) => {
            //                 //     let new_value = Value::Integer(999);
            //                 //     self.model.set_value(t.clone(), new_value);
            //                 //     self.trail.push(TrailElement::ModelAssignment(t.clone(), new_value))
            //                 // }
            //                 _ => panic!()
            //             }

            //         } else {
            //             // assert!(self.is_complete());
            //             return true
            //         }

            //     }
            //     State::Conflict(_) => {
            //         unimplemented!()
            //     }
            // }
        }
    }
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SOLVER\nstate:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}",
            self.state, self.trail, self.undecided
        )
    }
}
