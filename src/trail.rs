use crate::term::term::Term;
use crate::term::Value;

#[derive(Debug)]
pub struct Trail {
    elements: Vec<TrailElement>
}

impl Trail {
    pub fn new() -> Trail {
        Trail { elements: Vec::new() }
    }

    pub fn push(&mut self, element: TrailElement) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        self.elements.pop()
    }
}

#[derive(Debug)]
pub enum TrailElement {
    DecidedLiteral(Term),
    PropagatedLiteral(Term, Term),
    ModelAssignment(Term, Value)
}