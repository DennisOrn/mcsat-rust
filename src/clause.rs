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
