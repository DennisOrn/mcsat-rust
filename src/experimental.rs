/*
    Traits: Expression, (Term?), (Formula?)
    all terms and formulas are structs or enums, don't have a value
    terms have id

    hashconsing factory for each type:
    one for constants, one for variables, one for 'and', one for 'or'

    HashMap, key: expression (or term?), value: values
    one or several hashmaps? one is enough if expressions work as keys probably

    evaluate-function for each term/formula
    pass reference to hashmap as argument
    */



#[macro_use]
pub mod experimental {
    use hashconsing::*;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    use crate::model::Model;

    // pub type

    pub trait Expression {
        fn evaluate(&self, model: &Model) -> Option<bool>;
    }

    pub trait Term {}

    #[derive(Debug)]
    pub enum Value {
        Bool(bool),
        Integer(i32)
    }

    // pub struct HashconsFactory {
    //     boolean_factory: HConsign<Boolean>
    // }
    // impl HashconsFactory {
    //     pub fn new() -> HashconsFactory {
    //         HashconsFactory {
    //             boolean_factory: HConsign::empty()
    //         }
    //     }
    // }

    // #[derive(Debug)]
    // pub struct MapFactory { // TODO: is the name factory appropriate?
    //     pub map_boolean: HashMap<HConsed<Boolean>, Value>
    // }
    // impl MapFactory {
    //     pub fn new() -> MapFactory {
    //         MapFactory {
    //             map_boolean: HashMap::new()
    //         }
    //     }
    // }

    // consign! {
    //     let BOOLEAN_FACTORY = consign(37) for Boolean; // TODO: what does 37 mean?
    // }

    // pub fn foo() {
    //     // assert_eq! { BOOLEAN_FACTORY.len(), 0 }
    //     BOOLEAN_FACTORY.mk(Boolean { id: "x1".to_string() });
    //     // assert_eq! { BOOLEAN_FACTORY.len(), 1 }
    //     BOOLEAN_FACTORY.mk(Boolean { id: "x1".to_string() });
    //     // assert_eq! { BOOLEAN_FACTORY.len(), 1 }

    //     // let map_factory = MapFactory::new();
    //     // let mut map_boolean = map_factory.map_boolean;
    //     // let var = BOOLEAN_FACTORY.mk(Boolean { id: "x1".to_string() });
    //     // map_boolean.insert(var, Value::Bool(true));

    //     // println!("{:?}", map_boolean);
    // }
}




// pub struct Expressions {
    // pub type Expr<T> = HConsed<Expression<T>>;





    // pub trait Expression {
    //     fn evaluate(&self) -> Option<bool>;
    // }
    // // pub trait Term: Expression {}
    // // pub trait Formula: Expression {}

    // pub struct Boolean { id: String, value: Option<bool> }
    // impl Expression for Boolean {
    //     fn evaluate(&self) -> Option<bool> {
    //         self.value
    //     }
    // }

    // pub struct Or { operands: Vec<Box<dyn Expression>> }
    // impl Expression for Or {
    //     fn evaluate(&self) -> Option<bool> {
    //         for op in self.operands.iter() {
    //             match op.evaluate() {
    //                 Some(true) => return Some(true),
    //                 None       => return None,
    //                 _          => ()
    //             }
    //         }
    //         Some(false)
    //     }
    // }

    // pub struct Negation { expression: Box<dyn Expression> } // TODO: Term instead of Expression?
    // impl Expression for Negation {
    //     fn evaluate(&self) -> Option<bool> {
    //         match self.expression.evaluate() {
    //             Some(true)  => Some(false),
    //             Some(false) => Some(true),
    //             None        => None
    //         }
    //     }
    // }

    // pub fn boolean(id: &str, value: Option<bool>) -> Box<dyn Expression> {
    //     Box::new(Boolean { id: id.to_string(), value: value })
    // }

    // pub fn or(operands: Vec<Box<dyn Expression>>) -> Box<dyn Expression> {
    //     Box::new(Or { operands: operands})
    // }

    // pub fn negation(expression: Box<dyn Expression>) -> Box<dyn Expression> {
    //     Box::new(Negation { expression: expression })
    // }





    // #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    // pub struct Constant {
    //     value: i32
    // }

    // impl Expression for Constant {
    //     fn evaluate(&self) {
    //         println!("evaluate constant")
    //     }
    // }

    // consign! {
    //     // let FACTORY = consign(37) for dyn Expression; // TODO: what does 37 mean?
    //     let FACTORY = consign(37) for Box<dyn Expression>; // TODO: what does 37 mean?
    // }



    // pub fn foo(value: i32) /*-> Box<dyn Expression>*/ {
    //     // Box::new(Constant { value: value })

    //     // let mut factory: HConsign<Expression<T>> = HConsign::empty();
    //     // factory.mk(Expression::Expr(111));
    // }
// }