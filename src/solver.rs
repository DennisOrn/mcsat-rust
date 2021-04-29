use crate::clause::Clause;
use crate::literal::Literal;
use crate::state::State;
use crate::term::term::Term;
use crate::trail::Trail;
use crate::trail_element::TrailElement;
use crate::types::value::Value;
use crate::types::variable::Variable;
use colored::*;
use hashconsing::HConsed;
use rand::Rng;

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
        let mut rng = rand::thread_rng();
        loop {
            // Check which state we are in by evaluating clauses.
            self.state = State::Search;
            for clause in &self.clauses {
                print!("evaluate {}: ", clause);
                match clause.evaluate(self.trail.get_model()) {
                    Some(true) => println!("{}", "true".green()),
                    Some(false) => {
                        println!("{}", "false".red());
                        self.state = State::Conflict(clause.clone());
                        break;
                    }
                    None => println!("{}", "???".yellow()),
                }
            }

            println!("{}", self);

            match &self.state {
                State::Conflict(conflict_clause) => {
                    // Remove all model assignments from trail.
                    loop {
                        if let Some(TrailElement::ModelAssignment(var, _)) = self.trail.pop() {
                            self.undecided.push(var)
                        } else {
                            println!();
                            break;
                        }
                    }
                }
                State::Search => {
                    if self.undecided.len() == 0 {
                        return true;
                    }
                    let variable = self.undecided.pop().unwrap();
                    let value = Value::Integer(rng.gen_range(0..10));
                    self.trail.push_model_assignment(&variable, value);
                    println!("\nPush model assignment: {} = {}", variable, value);
                }
            }

            // Press enter for each step.
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let undecided: Vec<String> = self.undecided.iter().map(|x| x.to_string()).collect();
        write!(
            f,
            "SOLVER\nstate:\t\t{}\ntrail:\t\t{}\nundecided:\t{}\nis_consistent:\t{}\nis_complete:\t{}",
            self.state, self.trail, undecided.join(", "), self.trail.is_consistent(), self.trail.is_complete()
        )
    }
}
