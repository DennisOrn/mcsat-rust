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

enum Rule {
    // Decide,
    Propagate(Clause, Literal),
    Conflict(Clause),
    Sat,
    // Forget,
    Resolve(Clause),
    // Consume,
    Backjump(usize, Clause, Literal), // TODO: usize is number of popped trail elements
    Unsat,
    // Learn,
    TDecide(HConsed<Term>, Value),
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
            Rule::TDecide(var, val) => write!(f, "T-DECIDE: {}, {}", var, val),
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
        undecided: Vec<HConsed<Term>>, // TODO: should this be used differently?
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
                self.trail.push_propagated_literal(clause, literal)
            }
            Rule::Conflict(clause) => {
                println!("{}", "CONFLICT".blue());
                self.state = State::Conflict(clause.clone());
            }
            Rule::Sat => {
                println!("{}", "SAT".blue());
                self.state = State::Sat;
            }
            Rule::Resolve(clause) => {
                println!("{}", "RESOLVE".blue());
                self.trail.pop();
                self.state = State::Conflict(clause.clone());
            }
            Rule::Backjump(n, clause, literal) => {
                println!("{}", "BACKJUMP".blue());
                for _ in 0..*n {
                    self.trail.pop();
                }
                self.trail.push_propagated_literal(clause, literal);
                self.state = State::Search;
            }
            Rule::Unsat => {
                println!("{}", "UNSAT".blue());
                self.state = State::Unsat;
            }
            Rule::TDecide(variable, value) => {
                println!("{}", "T-DECIDE".blue());
                self.trail.push_model_assignment(variable.clone(), *value);
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
        if let State::Conflict(conflict) = &self.state {
            match self.trail.last() {
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
                                new_conflict = Clause::new(vec![Literal::new(f(), vec![], false)]);
                            }

                            rules.push(Rule::Resolve(new_conflict));
                            break;
                        }
                    }
                }
                _ => (),
            }
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
    fn print_applicable_rules(&self, rules: &Vec<Rule>) {
        let rules: Vec<String> = rules.iter().map(|x| x.to_string()).collect();
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
        self.print_applicable_rules(&self.get_applicable_rules());

        let variable = self.undecided.pop().unwrap();
        self.theory.decide(variable, &mut self.trail);
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        // self.propagate();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        // self.check_for_conflict();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        let conflict_clause_1: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_1 = conflict_clause.clone(),
            _ => panic!(),
        }
        // self.resolve(&conflict_clause_1);
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        self.backjump();
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        // self.propagate();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        // self.check_for_conflict();
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        let conflict_clause_2: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_2 = conflict_clause.clone(),
            _ => panic!(),
        }
        // self.resolve(&conflict_clause_2);
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

        let conflict_clause_3: Clause;
        match &self.state {
            State::Conflict(conflict_clause) => conflict_clause_3 = conflict_clause.clone(),
            _ => panic!(),
        }
        // self.resolve(&conflict_clause_3);
        self.apply(self.get_applicable_rules().first().unwrap());
        println!("{}", self);
        self.print_applicable_rules(&self.get_applicable_rules());

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

    pub fn run(&mut self) -> bool {
        // TODO: add clauses as decided literals here (?)

        // TODO: remove this
        self.apply(&Rule::TDecide(variable("x"), Value::True));
        self.undecided.remove(0);
        println!("{}", self);
        let _ = std::io::stdin().read_line(&mut String::new());

        loop {
            match self.state {
                State::Sat => return true,
                State::Unsat => return false,
                _ => (),
            }
            let rules = self.get_applicable_rules();
            self.print_applicable_rules(&rules);

            /* TEMPORARY; DELETE THIS */
            if rules.len() == 0 {
                self.apply(&Rule::Backjump(
                    1,
                    Clause::new(vec![Literal::new(
                        equal(variable("x"), constant(Value::True)),
                        vec![variable("x")],
                        true,
                    )]),
                    Literal::new(
                        equal(variable("x"), constant(Value::True)),
                        vec![variable("x")],
                        true,
                    ),
                ));
                println!("{}", self);
                let _ = std::io::stdin().read_line(&mut String::new());
                continue;
            }
            /* TEMPORARY; DELETE THIS */

            let rule = rules.first().unwrap(); // TODO: unsafe
            self.apply(&rule);

            println!("{}", self);

            // Press enter for each step.
            let _ = std::io::stdin().read_line(&mut String::new());
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
