mod clause;
mod decidable;
mod solver;
mod state;
mod theory;
mod trail;
mod variable;

use crate::clause::Clause;
use crate::solver::Solver;
use crate::theory::Boolean;
use crate::variable::Variable;

fn main() {

    let clause1 = Clause::new( vec![Variable::new(1, Boolean::Undef)] );
    let clause2 = Clause::new( vec![Variable::new(2, Boolean::Undef)] );
    let clause3 = Clause::new( vec![Variable::new(3, Boolean::Undef)] );

    let clauses = vec![clause1, clause2, clause3];
    print_clauses(&clauses);

    let undecided = vec![1, 2, 3];

    let mut solver = Solver::new(clauses, undecided);
    match solver.solve() {
        true => println!("SAT"),
        false => println!("UNSAT"),
    }
}

fn print_clauses(clauses: &Vec<Clause>) {
    println!("CLAUSES");
    for clause in clauses {
        println!("{:?}", clause);
    }
    println!();
}