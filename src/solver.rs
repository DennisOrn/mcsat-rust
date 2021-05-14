use crate::clause::Clause;
use crate::formula::formula::f;
use crate::literal::Literal;
use crate::state::State;
use crate::term::term::Term;
use crate::theory::Theory;
use crate::trail::Trail;
use crate::trail_element::TrailElement;
use colored::*;
use hashconsing::HConsed;

enum Rule {
    // Decide,
    Propagate(Clause, Literal),
    Conflict(Clause),
    Sat,
    // Forget,
    Resolve,
    // Consume,
    // Backjump,
    Unsat,
    // Learn,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rule::Propagate(clause, literal) => write!(f, "Propagate: {}, {}", clause, literal),
            Rule::Conflict(clause) => write!(f, "Conflict: {}", clause),
            Rule::Sat => write!(f, "Sat"),
            Rule::Resolve => write!(f, "Resolve"),
            Rule::Unsat => write!(f, "Unsat"),
        }
    }
}

// pub struct Solver<'a> {
pub struct Solver {
    theory: Box<dyn Theory>,
    state: State,
    trail: Trail,
    clauses: Vec<Clause>,
    undecided: Vec<HConsed<Term>>,
    // TODO: move this somewhere else?
    // TODO: rename
    //map: HashMap<Variable, Vec<&'a Literal>>,
}

// impl Solver<'_> {
impl Solver {
    pub fn new(
        theory: Box<dyn Theory>,
        clauses: Vec<Clause>,
        undecided: Vec<HConsed<Term>>,
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
        match rule {
            Rule::Propagate(clause, literal) => {
                println!("{}", "PROPAGATE".blue());
                self.trail.push_propagated_literal(&clause, &literal)
            }
            Rule::Conflict(clause) => {
                println!("{}", "CONFLICT".blue());
                self.state = State::Conflict(clause.clone());
            }
            Rule::Sat => {
                println!("{}", "SAT".blue());
                self.state = State::Sat;
            }
            Rule::Unsat => {
                println!("{}", "UNSAT".blue());
                self.state = State::Unsat;
            }
            _ => unimplemented!("Tried to apply unimplemented rule: {}", rule),
        }
    }

    fn get_applicable_rules(&self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = vec![];

        let mut conflict_clause: Option<Clause> = None;
        let mut propagation_clause: Option<Clause> = None;
        let mut propagation_literal: Option<Literal> = None;

        // TODO: skip this if we have a conflict.
        for clause in &self.clauses {
            // TODO: quit loop early if conflict/propagation found?

            // Check for conflict.
            match self.trail.value_clause(&clause) {
                Some(false) => conflict_clause = Some(clause.clone()),
                _ => (),
            }

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

        // RESOLVE
        let mut add_resolve = false;
        if let State::Conflict(conflict) = &self.state {
            match self.trail.last() {
                Some(TrailElement::PropagatedLiteral(_, literal)) => {
                    let negated = literal.negate();
                    // Check if negated propagated literal is in conflict clause.
                    for conflict_literal in conflict.get_literals() {
                        if conflict_literal == &negated {
                            add_resolve = true;
                        }
                    }
                }
                _ => (),
            }
        }

        if add_resolve {
            rules.push(Rule::Resolve);
        }

        // PROPAGATE
        // CONFLICT
        if self.state == State::Search {
            if let Some(clause) = propagation_clause {
                if let Some(literal) = propagation_literal {
                    rules.push(Rule::Propagate(clause, literal));
                }
            }
            if let Some(clause) = conflict_clause {
                rules.push(Rule::Conflict(clause));
            }
        }

        // SAT
        // TODO: can this be checked in the first loop?
        if self.trail.is_satisfied(&self.clauses) {
            rules.push(Rule::Sat);
        }

        // UNSAT
        match &self.state {
            State::Conflict(clause) => {
                // TODO: find another way to represent a "false" conflict clause.
                if clause == &Clause::new(vec![Literal::new(f(), vec![], false)]) {
                    rules.push(Rule::Unsat)
                }
            }
            _ => (),
        }

        // There must always be at least one applicable rule.
        // assert!(rules.len() > 0);
        rules
    }

    // Used for debugging.
    fn print_applicable_rules(&self) {
        let rules: Vec<String> = self
            .get_applicable_rules()
            .iter()
            .map(|x| x.to_string())
            .collect();
        if rules.len() == 0 {
            println!("{}", "NO RULES AVAILABLE".bright_purple());
        } else {
            println!(
                "{}\n{}",
                "RULES AVAILABLE:".bright_purple(),
                rules.join("\n").bright_purple()
            );
        }
    }

    pub fn run_hardcoded_example(&mut self) -> bool {
        // TODO: what about undecided?
        println!("{}", self);
        self.print_applicable_rules();

        let variable = self.undecided.pop().unwrap();
        self.theory.decide(variable, &mut self.trail);
        println!("{}", self);
        self.print_applicable_rules();

        // self.propagate();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules();

        // self.check_for_conflict();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules();

        let conflict_clause_1: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_1 = conflict_clause.clone(),
            _ => panic!(),
        }
        self.resolve(&conflict_clause_1);
        println!("{}", self);
        self.print_applicable_rules();

        self.backjump();
        println!("{}", self);
        self.print_applicable_rules();

        // self.propagate();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules();

        // self.check_for_conflict();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules();

        let conflict_clause_2: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_2 = conflict_clause.clone(),
            _ => panic!(),
        }
        self.resolve(&conflict_clause_2);
        println!("{}", self);
        self.print_applicable_rules();

        let conflict_clause_3: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_3 = conflict_clause.clone(),
            _ => panic!(),
        }
        self.resolve(&conflict_clause_3);
        println!("{}", self);
        self.print_applicable_rules();

        // let conflict_clause_4: Clause;
        // match &self.state {
        //     State::Conflict(conflict_clause) => conflict_clause_4 = conflict_clause.clone(),
        //     _ => panic!(),
        // }

        // assert_eq!(
        //     conflict_clause_4.get_literals()[0],
        //     Literal::new(f(), vec![], false)
        // );

        self.apply(self.get_applicable_rules().first().unwrap());
        assert!(self.state == State::Unsat);

        false
    }

    // pub fn run(&mut self) -> bool {
    //     // TODO: add clauses as decided literals here (?)
    //     // for clause in &self.clauses {
    //     //     let literals = clause.get_literals();
    //     //     for literal in literals {
    //     //         if self.trail.value_literal(literal).is_none() {
    //     //             println!("Push decided literal: {}", literal);
    //     //             self.trail.push_decided_literal(literal);
    //     //         }
    //     //     }
    //     // }

    //     // let mut rng = rand::thread_rng();
    //     for i in 1..1000 {
    //         println!("--------------- ITERATION {} ----------------", i);
    //         assert!(self.trail.is_consistent());

    //         // self.propagate();

    //         println!("\n{}\n", self);

    //         match &self.state {
    //             State::Conflict(conflict_clause) => {
    //                 println!("{}", "CONFLICT".blue());
    //                 // Remove all model assignments from trail.
    //                 // loop {
    //                 //     let element = self.trail.pop();
    //                 //     if let Some(TrailElement::ModelAssignment(var, _)) = element {
    //                 //         self.undecided.push(var);
    //                 //     } else if element.is_none() {
    //                 //         break;
    //                 //     }
    //                 // }

    //                 // assert!() // TODO: add assertion about conflict clause
    //                 // let literals = conflict_clause.get_literals();
    //                 // if literals.len() == 1 && literals[0] == Literal::new(f(), false) {
    //                 //     return false;
    //                 // }
    //                 // self.resolve(&conflict_clause.clone());
    //             }
    //             State::Search => {
    //                 // Check which state we are in by evaluating clauses.
    //                 self.state = State::Search;
    //                 for clause in &self.clauses {
    //                     print!("evaluate {}: ", clause);
    //                     match self.trail.value_clause(clause) {
    //                         Some(true) => {
    //                             println!("{}", "true".green())
    //                         }
    //                         Some(false) => {
    //                             println!("{}", "false".red());
    //                             self.state = State::Conflict(clause.clone());
    //                             break;
    //                         }
    //                         None => {
    //                             println!("{}", "undef".yellow());
    //                         }
    //                     }
    //                 }

    //                 match self.state {
    //                     State::Conflict(_) => {
    //                         // Press enter for each step.
    //                         let _ = std::io::stdin().read_line(&mut String::new());
    //                         continue;
    //                     }
    //                     _ => (),
    //                 }

    //                 if self.undecided.len() == 0 {
    //                     assert!(self.check_solution());
    //                     assert!(self.trail.is_complete());
    //                     return true;
    //                 }

    //                 let variable = self.undecided.pop().unwrap();

    //                 // let value = Value::Integer(rng.gen_range(0..2));

    //                 // let value: Value;
    //                 // match rng.gen_range(0..2) {
    //                 //     0 => value = Value::False,
    //                 //     1 => value = Value::True,
    //                 //     _ => panic!(),
    //                 // }

    //                 // let value = Value::False;
    //                 self.theory.decide(variable, &mut self.trail);
    //             }
    //         }

    //         // Press enter for each step.
    //         let _ = std::io::stdin().read_line(&mut String::new());
    //     }

    //     panic!()
    // }

    fn resolve(&mut self, conflict: &Clause) {
        println!("{}", "RESOLVE".blue());

        match self.trail.last() {
            Some(TrailElement::PropagatedLiteral(_, l)) => {
                let negated = l.negate();
                // Check if negated propagated literal is in conflict clause.
                for conflict_literal in conflict.get_literals() {
                    if conflict_literal == &negated {
                        println!("found negated literal: {}", &conflict_literal);
                        self.trail.pop();
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
                            new_conflict = Clause::new(vec![Literal::new(f(), vec![], false)]);
                        }

                        println!("new conflict clause: {}", &new_conflict);
                        self.state = State::Conflict(new_conflict);
                        return;
                    }
                }
            }
            _ => println!("Did not succeed, last element in trail != propagated literal"),
        }
    }

    fn backjump(&mut self) {
        println!("{}", "BACKJUMP".blue());

        let conflict: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict = conflict_clause.clone(),
            _ => panic!(),
        }

        loop {
            // Remove last trail element, if not decision: continue
            match self.trail.pop() {
                Some(TrailElement::DecidedLiteral(_)) => (),
                Some(TrailElement::ModelAssignment(_, _)) => (),
                None => {
                    println!("All elements removed from trail (is this supposed to happen?)");
                    return;
                }
                _ => continue,
            }

            let mut undefined = vec![];
            for literal in conflict.get_literals() {
                match self.trail.value_literal(&literal) {
                    Some(true) => break,
                    None => undefined.push(literal),
                    _ => (),
                }
            }

            if undefined.len() == 1 {
                println!("Push propagated literal: {} → {}", conflict, undefined[0]);
                self.trail.push_propagated_literal(&conflict, undefined[0]);
                self.state = State::Search;
                return;
            }
        }
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
