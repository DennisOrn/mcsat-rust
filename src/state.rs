use crate::clause::Clause;

#[derive(Debug)]
pub enum State {
    Search,
    ConflictResolution(Clause),
    Sat,
    Unsat,
}
