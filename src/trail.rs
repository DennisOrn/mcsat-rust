use crate::clause::Clause;
use crate::literal::Literal;
use crate::model::Model;
use crate::types::value::Value;
use crate::types::variable::Variable;

#[derive(Debug)]
pub struct Trail {
    elements: Vec<TrailElement>,
    model: Model,
}

impl Trail {
    pub fn new() -> Trail {
        Trail {
            elements: Vec::new(),
            model: Model::new(),
        }
    }

    pub fn push_decided_literal(&mut self, literal: Literal) {
        self.elements.push(TrailElement::DecidedLiteral(literal))
    }

    pub fn push_propagated_literal(&mut self, clause: Clause, literal: Literal) {
        self.elements
            .push(TrailElement::PropagatedLiteral(clause, literal))
    }

    pub fn push_model_assignment(&mut self, variable: Variable, value: Value) {
        self.elements
            .push(TrailElement::ModelAssignment(variable, value))
    }

    pub fn pop(&mut self) -> Option<TrailElement> {
        self.elements.pop()
    }

    pub fn value_b(&self, literal: &Literal) -> Option<bool> {
        println!("value_b with literal {}", literal);

        // TODO: inefficient to loop each time function is called.
        let literals: Vec<&Literal> = self
            .elements
            .iter()
            .filter(|x| match x {
                TrailElement::DecidedLiteral(_) => true,
                TrailElement::PropagatedLiteral(_, _) => true,
                _ => false,
            })
            .flat_map(|x| match x {
                TrailElement::DecidedLiteral(l) => Some(l),
                TrailElement::PropagatedLiteral(_, l) => Some(l),
                _ => None,
            })
            .collect();

        println!("literals in trail: ");
        for l in &literals {
            println!("{}", l);
        }

        let negated_literal = literal.negate();
        for l in literals {
            if l == literal {
                println!("{} == {}, return true\n", l, literal);
                return Some(true);
            } else if l == &negated_literal {
                println!("{} == {}, return false\n", l, negated_literal);
                return Some(false);
            }
        }

        println!("no match, return None\n");
        None
    }

    pub fn value_t(&self, literal: &Literal) -> Option<bool> {
        println!("value_t with literal {}", literal);

        // TODO: inefficient to loop each time function is called.
        let model_assignments: Vec<&TrailElement> = self
            .elements
            .iter()
            .filter(|x| match x {
                TrailElement::ModelAssignment(_, _) => true,
                _ => false,
            })
            .collect();

        let mut model_clone = self.model.clone();
        for assignment in model_assignments {
            if let TrailElement::ModelAssignment(var, val) = assignment {
                println!("updating model: {} = {}", var, val);
                model_clone.set_value(var.clone(), *val);
                match literal.evaluate(&model_clone) {
                    Some(true) => {
                        println!("evaluation returned true, return true\n");
                        return Some(true);
                    }
                    Some(false) => {
                        println!("evaluation returned false, return false\n");
                        return Some(false);
                    }
                    _ => (),
                }
                model_clone.clear_value(var.clone());
            }
        }

        println!("undefined, return None\n");
        None
    }

    // fn check_trail_element(&self, element: &TrailElement) -> bool {
    //     match element {
    //         TrailElement::DecidedLiteral(term) => {
    //             match term.get() {
    //                 ActualTerm::Boolean(_) => true,
    //                 _ => false
    //             }
    //         }
    //         TrailElement::ModelAssignment(var, val) => {
    //             match (var.get(), val) {
    //                 (ActualTerm::Variable(_), Value::Integer(_)) => true,
    //                 _ => false
    //             }
    //         }
    //         _ => panic!("PropagatedLiteral not implemented!")
    //     }
    // }
}

#[derive(Debug)]
pub enum TrailElement {
    DecidedLiteral(Literal),
    PropagatedLiteral(Clause, Literal),
    ModelAssignment(Variable, Value), // TODO: should Variable be HConsed?
}

impl std::fmt::Display for TrailElement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TrailElement::DecidedLiteral(f) => write!(fmt, "{}", f),
            TrailElement::PropagatedLiteral(c, f) => write!(fmt, "{} → {}", c, f),
            TrailElement::ModelAssignment(var, val) => write!(fmt, "{} ↦ {}", var, val),
        }
    }
}
