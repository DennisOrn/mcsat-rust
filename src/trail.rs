use crate::clause::Clause;
use crate::formula::formula::Formula;
use crate::term::term::Term;
use crate::term::term::Value;
use crate::term::term::Variable;
use hashconsing::*;

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

    pub fn value_b(&self, formula: &HConsed<Formula>) -> Option<bool> {
        // TODO: inefficient to loop each time function is called.
        let literals = self
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

        println!("literals:");
        for literal in literals {
            println!("{}", literal);
            if literal == formula {
                return Some(true);
            } // TODO: else if negation of formula != literal return Some(false)
        }

        None
    }

    pub fn value_t(&self, formula: &HConsed<Formula>) -> Option<bool> {
        /* PSEUDO CODE
        function value_t(formula) -> Option<bool> {
            for each model assignment (MA):
                replace variable in formula with the MA
                evaluate
                if true      => return true
                if false     => return false
                if undefined => continue loop
            return None
        }
        EXAMPLE:
        formula: x > 0
        MA: x → 1
        formula and MA implies that "1 > 0"
        evaluates to true: return Some(true)
        */

        // TODO: inefficient to loop each time function is called.
        let model_assignments = self.elements.iter().filter(|x| match x {
            TrailElement::ModelAssignment(_, _) => true,
            _ => false,
        });

        for assignment in model_assignments {
            println!("{}", assignment);
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

#[derive(Debug)]
pub enum TrailElement {
    DecidedLiteral(HConsed<Formula>),
    PropagatedLiteral(Clause, HConsed<Formula>),
    ModelAssignment(Variable, Value), // TODO: should Variable be HConsed?
}

impl ::std::fmt::Display for TrailElement {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TrailElement::DecidedLiteral(f) => write!(fmt, "{}", f),
            TrailElement::PropagatedLiteral(c, f) => write!(fmt, "{} → {}", c, f),
            TrailElement::ModelAssignment(var, val) => write!(fmt, "{} ↦ {}", var, val),
        }
    }
}
