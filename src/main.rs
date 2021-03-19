mod model;
mod solver;
mod state;
mod term;
mod trail;

use crate::solver::Solver;
use crate::term::term::*;

fn main() {

    // x < 1 ∨ p, ¬p ∨ x = 2
    let x = variable("x");
    let constant_1 = constant(1);
    let less_than = less_than(x, constant_1);
    let p = literal("p");
    let disjunction_1 = disjunction(less_than, p);

    println!("{}\n", disjunction_1);

    let mut solver = Solver::new(vec![disjunction_1],
                                 vec![variable("x"), literal("p")]);
    match solver.run() {
        true => println!("SAT"),
        false => println!("UNSAT")
    }
}

// // TODO: not completed
// fn evaluate(t: &term::Term, hash_map: &HashMap<term::Term, Value>) -> bool {
//     match t.get() {
//         ActualTerm::LessThan(lhs, rhs) => {
//             println!("LessThan({}, {})", lhs, rhs);
//             return lhs < rhs
//         },
//         ActualTerm::GreaterThan(lhs, rhs) => {
//             println!("GreaterThan({}, {})", lhs, rhs);
//             return lhs > rhs
//         },
//         ActualTerm::Equal(lhs, rhs) => {
//             println!("Equal({}, {})", lhs, rhs);
//             return lhs == rhs
//         },
//         ActualTerm::Conjunction(lhs, rhs) => {
//             println!("Conjunction({}, {})", lhs, rhs);
//             return evaluate(lhs, hash_map) && evaluate(rhs, hash_map)
//         },
//         ActualTerm::Disjunction(lhs, rhs) => {
//             println!("Disjunction({}, {})", lhs, rhs);
//             return evaluate(lhs, hash_map) || evaluate(rhs, hash_map)
//         },
//         _ => panic!("This is not supposed to happen...")
//     }
// }
