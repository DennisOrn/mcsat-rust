use crate::variable::Variable;

#[derive(Debug)]
pub struct Clause {
    variables: Vec<Variable>,
}

impl Clause {
    pub fn new(variables: Vec<Variable>) -> Clause {
        Clause { variables: variables}
    }

    // pub fn get_variables(&self) -> Vec<Variable> {
    //     self.variables.to_vec()
    // }
}