use crate::term::term::Formula;

#[derive(Debug)]
pub enum State {
    Search,
    Conflict(Formula)
}