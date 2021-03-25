use crate::term::term::Term;

#[derive(Debug)]
pub enum State {
    Search,
    Conflict(Term)
}