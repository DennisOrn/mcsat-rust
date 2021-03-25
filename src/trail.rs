use crate::term::term::Term;
use crate::term::term::ActualTerm;
use crate::term::Value;

#[derive(Debug)]
pub enum TrailElement {
    DecidedLiteral(Term),
    PropagatedLiteral(Term, Term),
    ModelAssignment(Term, Value)
}

#[derive(Debug)]
pub struct Trail {
    elements: Vec<TrailElement>
}

impl Trail {
    pub fn new() -> Trail {
        Trail { elements: Vec::new() }
    }

    pub fn push(&mut self, element: TrailElement) {
        assert!(self.check_trail_element(&element));
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        self.elements.pop()
    }


    fn check_trail_element(&self, element: &TrailElement) -> bool {
        match element {
            TrailElement::DecidedLiteral(term) => {
                match term.get() {
                    ActualTerm::Boolean(_) => true,
                    _ => false
                }
            }
            TrailElement::ModelAssignment(var, val) => {
                match (var.get(), val) {
                    (ActualTerm::Variable(_), Value::Integer(_)) => true,
                    _ => false
                }
            }
            _ => panic!("PropagatedLiteral not implemented!")
        }
    }
}