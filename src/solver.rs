use crate::clause::Clause;
use crate::formula::formula::{equal, f};
use crate::literal::Literal;
use crate::state::State;
use crate::term::term::{constant, variable, Term};
use crate::theory::Theory;
use crate::trail::Trail;
use crate::trail_element::TrailElement;
use crate::types::value::Value;
use colored::*;
use hashconsing::HConsed;
use std::collections::VecDeque;

enum Rule {
    Propagate(Clause, Literal),
    Conflict(Clause),
    Sat,
    Resolve(Clause),
    Backjump(usize, Clause, Literal), // TODO: usize is number of popped trail elements
    Unsat,
    TDecide(HConsed<Term>, Value),
    TConflict,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rule::Propagate(clause, literal) => write!(f, "PROPAGATE: {} → {}", clause, literal),
            Rule::Conflict(clause) => write!(f, "CONFLICT: {}", clause),
            Rule::Sat => write!(f, "SAT"),
            Rule::Resolve(clause) => write!(f, "RESOLVE: {}", clause),
            Rule::Backjump(n, clause, literal) => {
                write!(f, "BACKJUMP: {}, {}, {}", n, clause, literal)
            }
            Rule::Unsat => write!(f, "UNSAT"),
            Rule::TDecide(var, val) => write!(f, "T-DECIDE: {} ↦ {}", var, val),
            Rule::TConflict => write!(f, "T-CONFLICT"),
        }
    }
}

// pub struct Solver<'a> {
pub struct Solver {
    theory: Box<dyn Theory>,
    state: State,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: VecDeque<HConsed<Term>>,
    // TODO: move this somewhere else?
    // TODO: rename
    //map: HashMap<Variable, Vec<&'a Literal>>,
}

// impl Solver<'_> {
impl Solver {
    pub fn new(
        theory: Box<dyn Theory>,
        clauses: Vec<Clause>,
        undecided: VecDeque<HConsed<Term>>, // TODO: should this be used differently?
                                            //map: HashMap<Variable, Vec<&'a Literal>>,
    ) -> Solver {
        // Assert that all undecided terms are variables.
        for var in &undecided {
            assert!(matches!(var.get(), Term::Variable(_)))
        }
        Solver {
            theory: theory,
            state: State::Search,
            trail: Trail::new(),
            clauses: clauses,
            undecided: undecided,
            //map: map,
        }
    }

    fn apply(&mut self, rule: &Rule) {
        println!("{}", rule.to_string().blue());
        match rule {
            Rule::Propagate(clause, literal) => {
                self.trail.push_propagated_literal(clause, literal)
            }
            Rule::Conflict(clause) => {
                self.state = State::Conflict(clause.clone());
            }
            Rule::Sat => {
                self.state = State::Sat;
            }
            Rule::Resolve(clause) => {
                self.trail.pop();
                self.state = State::Conflict(clause.clone());
            }
            Rule::Backjump(n, clause, literal) => {
                assert!(n > &0);
                for _ in 0..*n {
                    match self.trail.pop() {
                        Some(TrailElement::ModelAssignment(variable, _)) => {
                            self.undecided.push_front(variable)
                        }
                        _ => (),
                    }
                }
                self.trail.push_propagated_literal(clause, literal);
                self.state = State::Search;
            }
            Rule::Unsat => {
                self.state = State::Unsat;
            }
            Rule::TDecide(variable, value) => {
                self.undecided.pop_front();
                self.trail.push_model_assignment(variable.clone(), *value);
            }
            Rule::TConflict => {
                let explanation = self.theory.conflict();
                self.state = State::Conflict(explanation);
            }
            // Rule::TConsume => {
            //     println!("{}", "T-CONSUME".blue());
            //     match self.trail.pop() {
            //         Some(TrailElement::ModelAssignment(variable, _)) => {
            //             self.undecided.push_front(variable)
            //         }
            //         _ => (),
            //     }
            // }
            _ => unimplemented!("Tried to apply unimplemented rule: {}", rule),
        }
    }

    // TODO: try separating all the rules into separate functions and run them concurrently, benchmark.

    fn get_available_rules(&self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = vec![];

        // SAT
        if self.trail.is_satisfied(&self.clauses) {
            rules.push(Rule::Sat);
        }

        match &self.state {
            State::Conflict(conflict) => {
                match self.trail.last() {

                    // TODO: broken, fix this!
                    // TODO: broken, fix this!
                    // TODO: broken, fix this!
                    // TODO: broken, fix this!
                    // TODO: broken, fix this!

                    // RESOLVE
                    Some(TrailElement::PropagatedLiteral(_, literal)) => {
                        let negated = literal.negate();
                        // Check if negated propagated literal is in conflict clause.
                        for conflict_literal in conflict.get_literals() {
                            if conflict_literal == &negated {
                                let remaining_literals: Vec<Literal> = conflict
                                    .get_literals()
                                    .iter()
                                    .filter(|l| l != &conflict_literal)
                                    .map(|l| l.clone())
                                    .collect();

                                let new_conflict: Clause;
                                if remaining_literals.len() > 0 {
                                    new_conflict = Clause::new(remaining_literals);
                                } else {
                                    new_conflict = Clause::new(vec![Literal::new(
                                        f(),
                                        /*vec![],*/ false,
                                    )]);
                                }

                                rules.push(Rule::Resolve(new_conflict));
                                break;
                            }
                        }
                    }

                    // T-CONSUME
                    // Remove model assignment if it doesn't affect the value of the conflict clause.
                    // TODO: inefficient.
                    // Some(TrailElement::ModelAssignment(_, _)) => {
                    //     let mut trail_clone = self.trail.clone();
                    //     trail_clone.pop();
                    //     if trail_clone.value_clause(conflict) == Some(false) {
                    //         rules.push(Rule::TConsume);
                    //     }
                    // }
                    _ => (),
                }

                // UNSAT
                // TODO: find another way to represent a "false" conflict clause.
                if conflict == &Clause::new(vec![Literal::new(f(), /*vec![],*/ false)]) {
                    rules.push(Rule::Unsat);
                }

                // BACKJUMP
                // TODO: potentially super inefficient!
                let mut trail_clone = self.trail.clone();
                let mut steps = 0;
                loop {
                    steps += 1;

                    // Remove last trail element, if not decision: continue
                    match trail_clone.pop() {
                        Some(TrailElement::DecidedLiteral(_)) => (),
                        Some(TrailElement::ModelAssignment(_, _)) => (),
                        None => break,
                        _ => continue,
                    }

                    let mut undefined = vec![];
                    for literal in conflict.get_literals() {
                        match trail_clone.value_literal(&literal) {
                            Some(true) => break,
                            None => undefined.push(literal),
                            _ => (),
                        }
                    }

                    if undefined.len() == 1 {
                        rules.push(Rule::Backjump(
                            steps,
                            conflict.clone(),
                            undefined[0].clone(),
                        ));
                        break;
                    }
                }
            }
            State::Search => {
                /* Place T-DECIDE first if last trail element is propagated literal,
                and the variable does not have a value. */
                // TODO: specific for boolean theory?

                // T-DECIDE
                // TODO: check if this really works.
                // if let Some(variable) = self.undecided.front() {
                //     let values = vec![Value::True, Value::False]; // TODO: hardcoded.
                //     for value in &values {
                //         let literal = Literal::new(
                //             equal(variable.clone(), constant(*value)),
                //             // vec![variable.clone()],
                //             false,
                //         );
                //         match self.trail.value_literal(&literal) {
                //             Some(true) | None => {
                //                 println!("able to t-decide: {} {}", variable.clone(), value);
                //                 rules.push(Rule::TDecide(variable.clone(), *value));
                //                 break;
                //             }
                //             _ => (),
                //         }
                //     }
                // }

                let mut conflict_clause: Option<Clause> = None;
                let mut propagation_clause: Option<Clause> = None;
                let mut propagation_literal: Option<Literal> = None;
                for clause in &self.clauses {
                    // TODO: quit loop early if conflict/propagation found?
                    if conflict_clause == None {
                        // Check for conflict.
                        match self.trail.value_clause(&clause) {
                            Some(false) => conflict_clause = Some(clause.clone()),
                            _ => (),
                        }
                    }

                    if propagation_clause == None {
                        // Check for propagation.
                        let mut skip = false;
                        let mut undefined = vec![];
                        for literal in clause.get_literals() {
                            match self.trail.value_literal(&literal) {
                                Some(true) => {
                                    skip = true;
                                    break;
                                }
                                None => undefined.push(literal),
                                _ => (),
                            }
                        }

                        if skip {
                            continue;
                        } else if undefined.len() == 1 {
                            propagation_clause = Some(clause.clone());
                            propagation_literal = Some(undefined[0].clone());
                        }
                    }
                }

                // PROPAGATE
                if let Some(clause) = propagation_clause {
                    if let Some(literal) = propagation_literal {
                        rules.push(Rule::Propagate(clause, literal));
                    }
                }

                // T-DECIDE
                if let Some(variable) = self.undecided.front() {
                    if let Some(value) = self.theory.decide(variable, &self.trail) {
                        rules.push(Rule::TDecide(variable.clone(), value))
                    }
                }

                // CONFLICT
                if let Some(clause) = conflict_clause {
                    rules.push(Rule::Conflict(clause));
                }
            }
            _ => (),
        }

        // Print available rules.
        let rules_list: Vec<String> = rules.iter().map(|x| x.to_string()).collect();
        if rules_list.len() == 0 {
            println!("{}", "NO RULES AVAILABLE".yellow());
        } else {
            println!(
                "{}\n{}",
                "RULES AVAILABLE:".yellow(),
                rules_list.join("\n").yellow()
            );
        }

        // There must always be at least one rule available.
        assert!(rules.len() > 0);

        rules
    }

    pub fn run_hardcoded(&mut self) -> bool {
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&Rule::TDecide(variable("y"), Value::True));
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&Rule::TConflict);
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);
        // self.apply(&self.get_available_rules().first().unwrap());
        // println!("{}", self);

        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);
        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);
        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);
        self.apply(&Rule::TConflict);
        println!("{}", self);
        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);
        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);
        self.apply(&self.get_available_rules().first().unwrap());
        println!("{}", self);

        false
    }

    pub fn run(&mut self) -> bool {
        // TODO: add clauses as decided literals here (?)

        let mut iteration_counter = 1;
        loop {
            assert!(self.trail.is_consistent());
            match self.state {
                State::Sat => {
                    assert!(self.trail.is_complete());
                    return true;
                }
                State::Unsat => return false,
                _ => (),
            }

            println!("ITERATION {}", iteration_counter);

            let rules = self.get_available_rules();
            let rule = rules.first().unwrap(); // TODO: potentially unsafe
            self.apply(&rule);

            println!("{}", self);

            // Press enter for each step.
            let _ = std::io::stdin().read_line(&mut String::new());

            iteration_counter += 1;
        }
    }

    fn color_boolean(&self, b: bool) -> ColoredString {
        if b {
            "true".green()
        } else {
            "false".red()
        }
    }
}

// impl std::fmt::Display for Solver<'_> {
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
