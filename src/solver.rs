use crate::clause::Clause;
use crate::model::Model;
use crate::state::State;
use crate::term::term::Term;
use crate::trail::Trail;
use hashconsing::HConsed;

#[derive(Debug)]
pub struct Solver {
    state: State,
    model: Model,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: Vec<HConsed<Term>>,
}

impl Solver {
    pub fn new(clauses: Vec<Clause>, undecided: Vec<HConsed<Term>>) -> Solver {
        Solver {
            state: State::Search,
            model: Model::new(),
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

    // fn is_consistent(&self) -> bool {
    //     println!("\nCHECK: is_consistent");
    //     for t in &self.clauses {
    //         match self.model.evaluate(&t) {
    //             Some(false) => { println!("{}: false", t); return false }
    //             Some(true) => { println!("{}: true", t); }
    //             None => { println!("{}: None", t); }
    //         }
    //     }
    //     true
    // }

    // fn is_complete(&self) -> bool {
    //     println!("\nCHECK: is_complete");
    //     for t in &self.clauses {
    //         match self.model.evaluate(&t) {
    //             Some(false) => { println!("{}: false", t); return false }
    //             Some(true) => { println!("{}: true", t); }
    //             None => { println!("{}: None", t); return false }
    //         }
    //     }
    //     true
    // }
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SOLVER\nstate:\t\t{:?}\nmodel:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}",
            self.state, self.model, self.trail, self.undecided
        )
    }
}
