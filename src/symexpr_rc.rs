#![allow(dead_code)]

use std::rc::Rc;
use std::collections::HashMap;
use std::ops::{Deref};
use numeric::{Numeric, RealType, ComplexType, IntegerType};

type ExprMap = HashMap<String, Rc<Expr>>;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Numeric(Numeric),
    IndepVar(usize),
    Unary(UnaryExpr),
    Binary(BinaryExpr)
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryFunction {
    Neg, Exp
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryFunction {
    Add, Mul
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryExpr {
    function: UnaryFunction,
    argument: Rc<Expr>
}

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryExpr {
    function: BinaryFunction,
    lhs: Rc<Expr>,
    rhs: Rc<Expr>
}

impl Expr {
    pub fn zero() -> Expr {
        Expr::from_integer(0)
    }

    pub fn from_numeric(val: Numeric) -> Expr {
        Expr::Numeric(val)
    }

    pub fn from_real(val: RealType) -> Expr {
        Expr::Numeric(Numeric::Real(val))
    }

    pub fn from_complex(val: ComplexType) -> Expr {
        Expr::Numeric(Numeric::Complex(val))
    }

    pub fn from_integer(val: IntegerType) -> Expr {
        Expr::Numeric(Numeric::Integer(val))
    }

    pub fn from_key(key: usize) -> Expr {
        Expr::IndepVar(key)
    }

    pub fn clone_to_heap(&self) -> Rc<Expr> {
        Rc::new(self.clone())
    }

    pub fn move_to_heap(self) -> Rc<Expr> {
        Rc::new(self)
    }

    fn is_unity(&self) -> bool {
        match self {
            Expr::Numeric(numeric) => return numeric.is_unity(),
            _ => return false
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Expr::Numeric(numeric) => return numeric.is_zero(),
            _ => return false
        }
    }

    pub fn unary_from_heap(argument: &Rc<Expr>, function: UnaryFunction) -> Expr {
        Expr::Unary(
            UnaryExpr{
                function: function,
                argument: Rc::clone(argument)
            }
        )
    }

    pub fn unary_from(argument: &Expr, function: UnaryFunction) -> Expr {
        Expr::Unary(
            UnaryExpr{
                function: function,
                argument: Rc::new(argument.clone())
            }
        )
    }

    pub fn binary_from_heap(lhs: &Rc<Expr>, rhs: &Rc<Expr>, function: BinaryFunction) -> Expr {
        Expr::Binary(
            BinaryExpr{
                function: function,
                lhs: Rc::clone(lhs),
                rhs: Rc::clone(rhs)
            }
        )
    }

    pub fn binary_from(lhs: &Expr, rhs: &Expr, function: BinaryFunction) -> Expr {
        Expr::Binary(
            BinaryExpr{
                function: function,
                lhs: Rc::new(lhs.clone()),
                rhs: Rc::new(rhs.clone())
            }
        )
    }

    pub fn eval(&self, values: &Vec<Numeric>) -> Numeric {
        match self {
            Expr::Numeric(numeric) => return numeric.clone(),
            Expr::IndepVar(key) => return values[*key],
            Expr::Unary(expr) => return expr.eval(values),
            Expr::Binary(expr) => return expr.eval(values)
        }
    }

    pub fn depends_on_any_variable(&self) -> bool {
        match self {
            Expr::IndepVar(_key) => return true,
            Expr::Unary(expr) => return expr.argument.depends_on_any_variable(),
            Expr::Binary(expr) => return expr.lhs.depends_on_any_variable() ||
                                         expr.rhs.depends_on_any_variable(),
            _ => return false
        }
    }

    pub fn depends_on_variable(&self, key: usize) -> bool {
        match self {
            Expr::IndepVar(this_key) => return *this_key == key,
            Expr::Unary(expr) => return expr.argument.depends_on_variable(key),
            Expr::Binary(expr) => return expr.lhs.depends_on_variable(key) ||
                                         expr.rhs.depends_on_variable(key),
            _ => return false
        }
    }

    fn trim(expr: &Rc<Expr>) -> Rc<Expr> {
        match expr.deref() {
            Expr::Unary(unary_exp) => {
                Expr::trim(&unary_exp.argument)
            },
            Expr::Binary(binary_expr) => {
                match binary_expr.trim() {
                    Some(binary_rc) => return binary_rc,
                    None => return Rc::clone(expr)
                }
            },
            _ => return Rc::clone(expr)
        }
    }
}

impl UnaryExpr {
    fn eval(&self, values: &Vec<Numeric>) -> Numeric {
        match self.function {
            UnaryFunction::Neg => return -self.argument.eval(values),
            UnaryFunction::Exp => return self.argument.eval(values).exp()
        }
    }
}

impl BinaryExpr {
    fn eval(&self, values: &Vec<Numeric>) -> Numeric {
        match self.function {
            BinaryFunction::Add => return self.lhs.eval(values) +
                                          self.rhs.eval(values),
            BinaryFunction::Mul => return self.lhs.eval(values) *
                                          self.rhs.eval(values)
        }
    }

    fn trim(&self) -> Option<Rc<Expr>> {
        match self.function {
            BinaryFunction::Add => {
                if self.lhs.is_zero() {
                    return Some(Rc::clone(&self.rhs))
                } else if self.rhs.is_zero() {
                    return Some(Rc::clone(&self.lhs))
                } else {
                    return None
                }
            },
            BinaryFunction::Mul => {
                if self.lhs.is_zero() || self.rhs.is_zero() {
                    return Some(Rc::new(Expr::zero()))
                } else if self.lhs.is_unity() {
                    return Some(Rc::clone(&self.rhs))
                } else if self.rhs.is_unity() {
                    return Some(Rc::clone(&self.lhs))
                } else {
                    return None
                }
            },
            _ => return None
        }
    }
}

pub fn neg(arg: &Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::unary_from_heap(&arg, UnaryFunction::Neg))
}

pub fn exp(arg: &Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::unary_from_heap(&arg, UnaryFunction::Exp))
}

pub fn add(lhs: &Rc<Expr>, rhs: &Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::binary_from_heap(&lhs, &rhs, BinaryFunction::Add))
}

pub fn sub(lhs: &Rc<Expr>, rhs: &Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::binary_from_heap(&lhs, &neg(&rhs), BinaryFunction::Add))
}

pub fn mul(lhs: &Rc<Expr>, rhs: &Rc<Expr>) -> Rc<Expr> {
    Rc::new(Expr::binary_from_heap(&lhs, &rhs, BinaryFunction::Mul))
}

#[cfg(test)]
#[test]
fn test_unity_or_zero() {
    assert!(Expr::from_real(1.).is_unity());
    assert!(Expr::from_integer(1).is_unity());
    assert!(Expr::from_real(0.).is_zero());
    assert!(Expr::from_integer(0).is_zero());
}

#[test]
fn test_expr_on_stack_and_heap() {
    let values: Vec<Numeric> = Vec::new();
    let a = Expr::from_integer(1);
    let b = Expr::from_integer(2);
    let c_on_heap = Expr::binary_from_heap(&a.clone_to_heap(), &b.clone_to_heap(), BinaryFunction::Add).clone_to_heap();
    let c_on_stack = Expr::binary_from(&a, &b, BinaryFunction::Add);
    assert_eq!(c_on_heap.eval(&values), Numeric::from_integer(3));
    assert_eq!(c_on_stack.eval(&values), Numeric::from_integer(3));
}

#[test]
fn test_evaluation_of_expr_with_variables() {
    let values: Vec<Numeric> = vec![
        Numeric::from_integer(2),
        Numeric::from_integer(3)
    ];
    let one = Expr::from_integer(1);
    let x = Expr::from_key(0);
    let y = Expr::from_key(1);

    // x + 1 + y
    let f = Expr::binary_from(
        &x, 
        &Expr::binary_from(
            &one,
            &y,
            BinaryFunction::Add
        ),
        BinaryFunction::Add
    );

    assert!(f.depends_on_any_variable());
    assert!(f.depends_on_variable(0));
    assert!(f.depends_on_variable(1));
    assert!(!f.depends_on_variable(2));

    assert_eq!(f.eval(&values), Numeric::from_integer(6));
}

#[test]
fn test_trimming() {
    let values: Vec<Numeric> = vec![
        Numeric::from_integer(2),
    ];

    let zero = Expr::from_integer(0);
    let two = Expr::from_integer(2);
    let x = Expr::from_key(0);

    let f = Expr::binary_from(
        &zero,
        &x,
        BinaryFunction::Add
    ).clone_to_heap();

    let g = Expr::binary_from(
        &two,
        &two,
        BinaryFunction::Add
    ).clone_to_heap();

    assert_eq!(*Expr::trim(&f), Expr::IndepVar(0));
    assert_eq!(*Expr::trim(&g), *g);
}

#[test]
fn test_arithmetic() {
    let values: Vec<Numeric> = Vec::new();
    let a = Expr::from_integer(2).clone_to_heap();
    let b = Expr::from_integer(3).clone_to_heap();

    assert_eq!(add(&a, &b).eval(&values), Numeric::from_integer(5));
    assert_eq!(sub(&a, &b).eval(&values), Numeric::from_integer(-1));
    assert_eq!(mul(&a, &b).eval(&values), Numeric::from_integer(6));
}