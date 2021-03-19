use crate::term::term::Term;

#[derive(Debug)]
pub enum State {
    Consistent,
    Conflict(Term)
}
