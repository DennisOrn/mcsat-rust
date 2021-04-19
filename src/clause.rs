use crate::literal::Literal;
use hashconsing::HConsed;

#[derive(Debug)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Clause {
        Clause { literals: literals }
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formulas_list: Vec<String> = self.literals.iter().map(|x| x.to_string()).collect();
        write!(fmt, "{}", formulas_list.join(" ∨ "))
    }
}
