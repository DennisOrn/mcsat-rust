use crate::clause::Clause;
use crate::formula::formula::Formula;
use crate::term::term::Term;
use crate::term::term::Value;
use crate::term::term::Variable;
use hashconsing::*;

#[derive(Debug)]
pub enum TrailElement {
    DecidedLiteral(Formula),
    PropagatedLiteral(Clause, Formula),
    ModelAssignment(Variable, Value),
}

#[derive(Debug)]
pub struct Trail {
    elements: Vec<TrailElement>,
}

impl Trail {
    pub fn new() -> Trail {
        Trail {
            elements: Vec::new(),
        }
    }

    // TODO: change the name of these functions?
    pub fn value_b(&self, formula: &HConsed<Formula>) -> Option<bool> {
        // TODO: inefficient to loop each time function is called.
        let decided_literals = self
            .elements
            .iter()
            .filter(|x| match x {
                TrailElement::DecidedLiteral(_) => true,
                TrailElement::PropagatedLiteral(_, _) => true,
                _ => false,
            })
            .flat_map(|x| match x {
                TrailElement::DecidedLiteral(l) => Some(l),
                TrailElement::PropagatedLiteral(_, l) => Some(l),
                _ => None,
            });

        let f = formula.get();

        println!("decided literals:");
        for l in decided_literals {
            println!("{:?}", l);
            if f == l {
                return Some(true);
            } // TODO: else if negation of f != l return Some(false)
        }

        None
    }

    pub fn value_t(&self, formula: &HConsed<Formula>) -> Option<bool> {
        /* PSEUDO
        for each model assignment
        */

        // TODO: inefficient to loop each time function is called.
        let model_assignments = self.elements.iter().filter(|x| match x {
            TrailElement::ModelAssignment(_, _) => true,
            _ => false,
        });

        for x in model_assignments {
            println!("{:?}", x);
        }

        None // TODO: return correct value
    }

    pub fn push(&mut self, element: TrailElement) {
        // assert!(self.check_trail_element(&element));
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        self.elements.pop()
    }

    // fn check_trail_element(&self, element: &TrailElement) -> bool {
    //     match element {
    //         TrailElement::DecidedLiteral(term) => {
    //             match term.get() {
    //                 ActualTerm::Boolean(_) => true,
    //                 _ => false
    //             }
    //         }
    //         TrailElement::ModelAssignment(var, val) => {
    //             match (var.get(), val) {
    //                 (ActualTerm::Variable(_), Value::Integer(_)) => true,
    //                 _ => false
    //             }
    //         }
    //         _ => panic!("PropagatedLiteral not implemented!")
    //     }
    // }
}
