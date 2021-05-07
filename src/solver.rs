use crate::clause::Clause;
use crate::formula::formula::f;
use crate::literal::Literal;
use crate::state::State;
use crate::term::term::Term;
use crate::theory::Theory;
use crate::trail::Trail;
use crate::trail_element::TrailElement;
use crate::types::value::Value;
use crate::types::variable::Variable;
use colored::*;
use rand::Rng;

pub struct Solver {
    theory: Box<dyn Theory>,
    state: State,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: Vec<Variable>,
}

impl Solver {
    pub fn new(theory: Box<dyn Theory>, clauses: Vec<Clause>, undecided: Vec<Variable>) -> Solver {
        Solver {
            theory: theory,
            state: State::Search,
            trail: Trail::new(),
            clauses: clauses,
            undecided: undecided,
        }
    }

    pub fn run(&mut self) -> bool {
        // TODO: add clauses as decided literals here (?)
        // for clause in &self.clauses {
        //     let literals = clause.get_literals();
        //     for literal in literals {
        //         if self.trail.value_literal(literal).is_none() {
        //             println!("Push decided literal: {}", literal);
        //             self.trail.push_decided_literal(literal);
        //         }
        //     }
        // }

        let mut rng = rand::thread_rng();
        for i in 1..1000 {
            println!("--------------- ITERATION {} ----------------", i);
            assert!(self.trail.is_consistent());

            self.propagate();

            println!("\n{}\n", self);

            match &self.state {
                State::Conflict(conflict_clause) => {
                    // Remove all model assignments from trail.
                    // loop {
                    //     let element = self.trail.pop();
                    //     if let Some(TrailElement::ModelAssignment(var, _)) = element {
                    //         self.undecided.push(var);
                    //     } else if element.is_none() {
                    //         break;
                    //     }
                    // }

                    // assert!() // TODO: add assertion about conflict clause
                    // let literals = conflict_clause.get_literals();
                    // if literals.len() == 1 && literals[0] == Literal::new(f(), false) {
                    //     return false;
                    // }
                    // self.resolve(&conflict_clause.clone());
                }
                State::Search => {
                    // Check which state we are in by evaluating clauses.
                    self.state = State::Search;
                    for clause in &self.clauses {
                        print!("evaluate {}: ", clause);
                        match self.trail.value_clause(clause) {
                            Some(true) => {
                                println!("{}", "true".green())
                            }
                            Some(false) => {
                                println!("{}", "false".red());
                                self.state = State::Conflict(clause.clone());
                                break;
                            }
                            None => {
                                println!("{}", "undef".yellow());
                            }
                        }
                    }

                    match self.state {
                        State::Conflict(_) => continue,
                        _ => (),
                    }

                    if self.undecided.len() == 0 {
                        assert!(self.check_solution());
                        assert!(self.trail.is_complete());
                        return true;
                    }

                    let variable = self.undecided.pop().unwrap();

                    // let value = Value::Integer(rng.gen_range(0..2));

                    // let value: Value;
                    // match rng.gen_range(0..2) {
                    //     0 => value = Value::False,
                    //     1 => value = Value::True,
                    //     _ => panic!(),
                    // }

                    // let value = Value::False;
                    self.theory.decide(&variable, &mut self.trail);
                }
            }

            // Press enter for each step.
            let _ = std::io::stdin().read_line(&mut String::new());
        }

        panic!()
    }

    // BCP
    fn propagate(&mut self) {
        loop {
            println!("{}", "PROPAGATE".blue());
            let mut run_again = false;
            for clause in &self.clauses {
                if self.trail.value_clause(&clause) != None {
                    continue;
                } //TODO

                let undefined_literals: Vec<&Literal> = clause
                    .get_literals()
                    .iter()
                    .filter(|l| self.trail.value_literal(&l).is_none())
                    .collect();

                if undefined_literals.len() == 1 {
                    println!(
                        "Push propagated literal: {} → {}",
                        clause, undefined_literals[0]
                    );
                    self.trail
                        .push_propagated_literal(&clause, undefined_literals[0]);
                    run_again = true;
                }
            }

            if !run_again {
                println!("Done propagating\n");
                return;
            }
        }
    }

    // fn resolve(&mut self, conflict: &Clause) {
    //     println!("{}", "RESOLVE".blue());

    //     match self.trail.last() {
    //         Some(TrailElement::PropagatedLiteral(c, l)) => {
    //             let negated = l.negate();
    //             for conflict_literal in conflict.get_literals() {
    //                 if conflict_literal == &negated {
    //                     self.trail.pop();
    //                     // TODO: hardcoded
    //                     self.state = State::Conflict(Clause::new(vec![Literal::new(f(), false)]))
    //                 }
    //             }
    //         }
    //         _ => println!("Did not succeed, last element in trail != propagated literal"),
    //     }
    // }

    fn consume(&self) {
        unimplemented!()
    }

    fn check_solution(&self) -> bool {
        for clause in &self.clauses {
            match self.trail.value_clause(clause) {
                Some(false) | None => return false,
                _ => (),
            }
        }
        true
    }

    fn color_boolean(&self, b: bool) -> ColoredString {
        if b {
            "true".green()
        } else {
            "false".red()
        }
    }
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let undecided: Vec<String> = self.undecided.iter().map(|x| x.to_string()).collect();
        let is_consistent = self.color_boolean(self.trail.is_consistent());
        let is_complete = self.color_boolean(self.trail.is_complete());
        write!(
            f,
            "SOLVER\nstate:\t\t{}\ntrail:\t\t{}\nundecided:\t{}\nis_consistent:\t{}\nis_complete:\t{}",
            self.state, self.trail, undecided.join(", "), is_consistent, is_complete
        )
    }
}
