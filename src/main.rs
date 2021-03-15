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

    hashcons_test()

    // let clause1 = Clause::new( vec![Variable::new(1, Boolean::Undef)] );
    // let clause2 = Clause::new( vec![Variable::new(2, Boolean::Undef)] );
    // let clause3 = Clause::new( vec![Variable::new(3, Boolean::Undef)] );

    // let clauses = vec![clause1, clause2, clause3];
    // print_clauses(&clauses);

    // let undecided = vec![1, 2, 3];

    // let mut solver = Solver::new(clauses, undecided);
    // match solver.solve() {
    //     true => println!("SAT"),
    //     false => println!("UNSAT"),
    // }
}

fn print_clauses(clauses: &Vec<Clause>) {
    println!("CLAUSES");
    for clause in clauses {
        println!("{:?}", clause);
    }
    println!();
}


use hashconsing:: { HConsed, HashConsign, HConsign };

type Term = HConsed<ActualTerm>;
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum ActualTerm {
    Var(usize),
    Lam(Term),
    App(Term, Term)
}
use ActualTerm::*;

fn hashcons_test() {
    let mut factory: HConsign<ActualTerm> = HConsign::empty();
    assert_eq! { factory.len(), 0 }

    let v = factory.mk(Var(0));
    assert_eq! { factory.len(), 1 }

    let v2 = factory.mk(Var(3));
    assert_eq! { factory.len(), 2 }

    let lam = factory.mk(Lam(v2.clone()));
    assert_eq! { factory.len(), 3 }

    let v3 = factory.mk(Var(3));
    assert_eq! { factory.len(), 3 }
    assert_eq! { v2.uid(), v3.uid() }
    assert_eq! { v2.get(), v3.get() }
    assert_eq! { v2, v3 }

    let lam2 = factory.mk(Lam(v3));
    assert_eq! { factory.len(), 3 }
    assert_eq! { lam.uid(), lam2.uid() }
    assert_eq! { lam.get(), lam2.get() }
    assert_eq! { lam, lam2 }

    let app = factory.mk(App(lam2, v));
    assert_eq! { factory.len(), 4 }
}