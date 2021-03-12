use crate::decidable::Decidable;
use crate::theory::Boolean;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    id: i32,
    value: Boolean,
}

impl Variable {
    pub fn new(id: i32, value: Boolean) -> Variable {
        Variable { id: id, value: value }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_value(&self) -> &Boolean {
        &self.value
    }

    pub fn set_value(&mut self, value: Boolean) {
        self.value = value
    }
}

// impl<T> Decidable<T> for Variable<T> {
//     fn get_id(&self) -> i32 {
//         self.id
//     }

//     fn get_value(&self) -> &T {
//         &self.value
//     }

//     fn set_value(&mut self, value: T) {
//         self.value = value
//     }
// }