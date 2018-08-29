#![allow(dead_code)]

use std::collections::{HashMap};

// Placeholder typedefs until later attempt at making module generic
type ComplexType = f64;
type RealType = f64;
type IntegerType = i64;

// Numeric type
enum Numeric {
    Real(RealType),
    Complex(ComplexType),
    Integer(IntegerType)
}

// Expression
enum Expr {
    Numeric(Numeric),
    Variable(String)
}

impl Expr {
    fn depends_on_any_variable(&self) -> bool {
        match self {
            Expr::Numeric(_num) => return false,
            Expr::Variable(_key) => return true
        }
    }

    fn depends_on_variable(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> bool {
        match self {
            Expr::Numeric(_num) => return false,
            Expr::Variable(this_key) => {
                match expr_map.get(this_key) {
                    Some(expr) => expr.depends_on_variable(expr_key, expr_map),
                    None => return false
                }
            }
        }
    }
}

