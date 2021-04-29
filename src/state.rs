use crate::clause::Clause;

#[derive(Debug)]
pub enum State {
    Search,
    Conflict(Clause),
}

impl std::fmt::Display for State {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Search => write!(fmt, "Search"),
            State::Conflict(clause) => write!(fmt, "Conflict: {}", clause),
        }
    }
}
