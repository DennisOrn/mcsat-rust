#[macro_use]
pub mod expression {
    use hashconsing::*;
    // pub type Expr = HConsed<dyn Expression>;

    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};



    pub trait Expression: CloneExpression + Send + Sync {
        fn evaluate(&self);
    }

    pub trait CloneExpression {
        fn clone_expression<'a>(&self) -> Box<dyn Expression>;
    }

    impl PartialEq for dyn Expression {
        fn eq(&self, rhs: &Self) -> bool {
            self == rhs
        }
    }
    impl Eq for dyn Expression {}
    impl Hash for dyn Expression {
        fn hash<H>(&self, state: &mut H) where H: Hasher {
            self.hash(state)
        }
    }
    impl<T> CloneExpression for T where T: Expression + Clone + 'static {
        fn clone_expression(&self) -> Box<dyn Expression> {
            Box::new(self.clone())
        }
    }
    impl Clone for Box<dyn Expression> {
        fn clone(&self) -> Self {
            self.clone_expression()
        }
    }


    // pub struct Storage {
    //     pub expressions: Vec<Box<dyn Expression>>
    // }

    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    pub struct Constant {
        value: i32
    }

    impl Expression for Constant {
        fn evaluate(&self) {
            println!("evaluate constant")
        }
    }

    // consign! {
    //     // let FACTORY = consign(37) for dyn Expression; // TODO: what does 37 mean?
    //     let FACTORY = consign(37) for Box<dyn Expression>; // TODO: what does 37 mean?
    // }

    pub fn constant(value: i32) /*-> Box<dyn Expression>*/ {
        // Box::new(Constant { value: value })

        let mut factory: HConsign<Box<dyn Expression>> = HConsign::empty();

        factory.mk(Box::new(Constant { value: 111 }));

    }
}