use crate::clause::Clause;

#[derive(PartialEq)]
pub enum State {
    Search,
    Conflict(Clause),
    Sat,
    Unsat,
}

impl std::fmt::Display for State {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Search => write!(fmt, "Search"),
            State::Conflict(clause) => write!(fmt, "Conflict: {}", clause),
            State::Sat => write!(fmt, "SAT"),
            State::Unsat => write!(fmt, "UNSAT"),
        }
    }
}
