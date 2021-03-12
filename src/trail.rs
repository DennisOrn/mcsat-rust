use crate::variable::Variable;

#[derive(Debug)]
pub struct Trail {
    elements: Vec<Variable>, // TODO: Clause instead?
}

impl Trail {
    pub fn new() -> Trail {
        Trail { elements: Vec::new() }
    }

    pub fn push(&mut self, element: Variable) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<Variable> {
        self.elements.pop()
    }


    // fn is_consistent() -> bool {
    //     unimplemented!("function is not implemented")
    // }

    // fn is_complete() -> bool {
    //     unimplemented!("function is not implemented")
    // }

    // fn is_satisfied() -> bool {
    //     unimplemented!("function is not implemented")
    // }

    // fn is_infeasible() -> bool {
    //     unimplemented!("function is not implemented")
    // }

    // fn value_b(&self, literal: Literal) -> Option<bool> {
    //     if self.elements.contains(&literal) {
    //         return Some(true)
    //     } else if self.elements.contains(&literal.negate()) {
    //         return Some(false)
    //     } else {
    //         return None
    //     }
    // }
}

// impl fmt::Display for Trail {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,
//             "{}",
//             self.elements)
//     }
// }