use crate::clause::Clause;

#[derive(Debug)]
pub enum State {
    Search,
    Conflict(Clause),
}
