use crate::formula::formula::Formula;
use hashconsing::HConsed;

#[derive(Debug)]
pub struct Clause {
    formulas: Vec<HConsed<Formula>>,
}

impl Clause {
    pub fn new(formulas: Vec<HConsed<Formula>>) -> Clause {
        Clause { formulas: formulas }
    }
}

impl ::std::fmt::Display for Clause {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let formulas_list: Vec<String> = self.formulas.iter().map(|x| x.to_string()).collect();
        write!(fmt, "{}", formulas_list.join(" ∨ "))
    }
}
