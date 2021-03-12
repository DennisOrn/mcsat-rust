use std::fmt;
use crate::clause::Clause;
use crate::state::State;
use crate::theory::Boolean;
use crate::trail::Trail;
use crate::variable::Variable;

#[derive(Debug)]
pub struct Solver {
    state: State,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: Vec<i32>,
}

impl Solver {
    pub fn new(clauses: Vec<Clause>, undecided: Vec<i32>) -> Solver {
        Solver {
            state: State::Search,
            trail: Trail::new(),
            clauses: clauses,
            undecided: undecided,
        }
    }

    pub fn solve(&mut self) -> bool {
        loop {
            println!("{}", self);

            self.propagate();

            match &self.state {

                State::ConflictResolution(_conflict_clause) => {
                    unimplemented!("Do something cool here")
                }

                State::Search => {
                    if let Some(id) = self.undecided.pop() {
                        self.decide(id)

                    } else {
                        self.state = State::Sat;
                        return true
                    }
                }

                _ => (),
            }
        }
    }

    fn decide(&mut self, id: i32) {
        let trail_element = Variable::new(id, Boolean::True);
        self.trail.push(trail_element);
    }

    fn propagate(&mut self) {
        return
    }

    // TODO: check if all clauses are satisifed by the trail
    // fn satisfied(&self) -> bool {
    //     self.undecided.is_empty()
    // }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "SOLVER\nstate:\t\t{:?}\ntrail:\t\t{:?}\nundecided:\t{:?}\n",
            self.state, self.trail, self.undecided)
    }
}