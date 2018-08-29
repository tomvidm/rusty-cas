#![allow(dead_code)]

use std::collections::{HashMap};
use std::ops::{Neg};

use numeric::{Numeric, RealType, ComplexType, IntegerType};

// Expression
#[derive(Clone, PartialEq, Debug)]
enum Expr {
    Numeric(Numeric),
    Power(Power),
    Variable(String),
    Unary(Unary),
    Binary(Binary)
}

#[derive(Clone, PartialEq, Debug)]
struct Power {
    pow: IntegerType,
    base: Box<Expr>
}

#[derive(Clone, PartialEq, Debug)]
enum UnaryFunction {
    Neg
}

#[derive(Clone, PartialEq, Debug)]
struct Unary {
    function: UnaryFunction,
    argument: Box<Expr>
}

#[derive(Clone, PartialEq, Debug)]
enum BinaryFunction {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, PartialEq, Debug)]
struct Binary {
    function: BinaryFunction,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Expr {
    fn from_real(val: RealType) -> Expr {
        Expr::Numeric(Numeric::Real(val))
    }

    fn from_complex(val: ComplexType) -> Expr {
        Expr::Numeric(Numeric::Complex(val))
    }

    fn from_integer(val: IntegerType) -> Expr {
        Expr::Numeric(Numeric::Integer(val))
    }

    fn from_key(key: &String) -> Expr {
        Expr::Variable(key.clone())
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

    fn pow_from(base: &Expr, pow: IntegerType) -> Expr {
        Expr::Power(
            Power{
                pow: pow,
                base: Box::new(base.clone())
            }
        )
    }

    fn unary_from(argument: &Expr, function: UnaryFunction) -> Expr {
        Expr::Unary(
            Unary{
                function: function,
                argument: Box::new(argument.clone())
            }
        )
    }

    fn binary_from(lhs: &Expr, rhs: &Expr, function: BinaryFunction) -> Expr {
        Expr::Binary(
            Binary{
                function: function,
                lhs: Box::new(lhs.clone()),
                rhs: Box::new(rhs.clone())
            }
        )
    }

    fn eval(&self, expr_map: &HashMap<String, Box<Expr>>) -> Numeric {
        match self {
            Expr::Numeric(number) => return number.clone(),
            Expr::Variable(key) => {
                match expr_map.get(key) {
                    Some(expr) => return expr.eval(&expr_map),
                    None => return Numeric::zero()
                }
            },
            Expr::Power(power) => return power.eval(expr_map),
            Expr::Unary(unary) => return unary.eval(expr_map),
            Expr::Binary(binary) => return binary.eval(expr_map)
        }
    }

    fn depends_on_any_variable(&self) -> bool {
        match self {
            Expr::Numeric(_num) => return false,
            Expr::Variable(_key) => return true,
            Expr::Power(power) => return power.base.depends_on_any_variable(),
            Expr::Unary(unary) => return unary.argument.depends_on_any_variable(),
            Expr::Binary(binary) => return binary.lhs.depends_on_any_variable() ||
                                           binary.rhs.depends_on_any_variable()
        }
    }

    fn depends_on_variable(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> bool {
        match self {
            Expr::Numeric(_num) => return false,
            Expr::Variable(this_key) => {
                if expr_key == this_key { return true }
                match expr_map.get(this_key) {
                    Some(expr) => expr.depends_on_variable(expr_key, expr_map),
                    None => return false
                }
            },
            Expr::Power(power) => return power.base.depends_on_variable(expr_key, expr_map),
            Expr::Unary(unary) => return unary.argument.depends_on_variable(expr_key, expr_map),
            Expr::Binary(binary) => return binary.lhs.depends_on_variable(expr_key, expr_map) ||
                                           binary.rhs.depends_on_variable(expr_key, expr_map)
        }
    }

    fn get_cleaned(&self) -> Expr {
        match self {
            Expr::Binary(binary) => {
                return binary.clone().get_cleaned()
            },
            Expr::Unary(unary) => {
                return unary.clone().get_cleaned()
            }
            _ => return self.clone()
        }
    }

    fn get_derivative(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> Expr {
        if !self.depends_on_variable(expr_key, expr_map) {
            return Expr::from_integer(0)
        } else {
            match self {
                Expr::Numeric(_number) => return Expr::from_integer(0),
                Expr::Variable(key) => {
                    match expr_map.get(key) {
                        Some(expr) => {
                            if expr.depends_on_variable(expr_key, expr_map) {
                                return expr.get_derivative(expr_key, expr_map).get_cleaned()
                            } else {
                                return Expr::from_integer(1)
                            }
                        },
                        None => Expr::from_integer(0)
                    }
                },
                Expr::Power(power) => return power.get_derivative(expr_key, expr_map).get_cleaned(),
                Expr::Unary(unary) => return unary.get_derivative(expr_key, expr_map).get_cleaned(),
                Expr::Binary(binary) => return binary.get_derivative(expr_key, expr_map).get_cleaned()
            }
        }
    }

    fn neg(&self) -> Expr {
        return Expr::unary_from(self, UnaryFunction::Neg)
    }

    fn add(&self, other: &Expr) -> Expr {
        return Expr::binary_from(self, other, BinaryFunction::Add)
    }

    fn sub(&self, other: &Expr) -> Expr {
        return Expr::binary_from(self, other, BinaryFunction::Sub)
    }

    fn mul(&self, other: &Expr) -> Expr {
        return Expr::binary_from(self, other, BinaryFunction::Mul)
    }

    fn div(&self, other: &Expr) -> Expr {
        return Expr::binary_from(self, other, BinaryFunction::Div)
    }
    
    fn pow(&self, pow: IntegerType) -> Expr {
        return Expr::pow_from(self, pow)
    }
}

impl Power {
    fn eval(&self, expr_map: &HashMap<String, Box<Expr>>) -> Numeric {
        self.base.eval(expr_map).pow(self.pow)
    }

    fn get_derivative(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> Expr {
        if self.pow > 1 {
            return Expr::binary_from(
                &Expr::binary_from(
                    &Expr::from_integer(self.pow),
                    &self.base.get_derivative(expr_key, expr_map),
                    BinaryFunction::Mul
                ),
                &Expr::pow_from(
                    &self.base.clone(),
                    self.pow - 1
                ),
                BinaryFunction::Mul
            )
        } else {
            return Expr::from_integer(0)
        }
    }
}

impl Unary {
    fn eval(&self, expr_map: &HashMap<String, Box<Expr>>) -> Numeric {
        match self.function {
            UnaryFunction::Neg => return -self.argument.eval(expr_map)
        }
    }

    fn get_cleaned(self) -> Expr {
        match self.function {
            _ => return Expr::unary_from(&self.argument.get_cleaned(), self.function)
        }
    }

    fn get_derivative(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> Expr {
        match self.function {
            UnaryFunction::Neg => {
                return Expr::unary_from(&self.argument.get_derivative(expr_key, expr_map), UnaryFunction::Neg)
            }
        }
    }
}

impl Binary {
    fn eval(&self, expr_map: &HashMap<String, Box<Expr>>) -> Numeric {
        match self.function {
            BinaryFunction::Add => return self.lhs.eval(expr_map) + self.rhs.eval(expr_map),
            BinaryFunction::Sub => return self.lhs.eval(expr_map) - self.rhs.eval(expr_map),
            BinaryFunction::Mul => return self.lhs.eval(expr_map) * self.rhs.eval(expr_map),
            BinaryFunction::Div => return self.lhs.eval(expr_map) / self.rhs.eval(expr_map)
        }
    }

    fn get_cleaned(self) -> Expr {
        match self.function {
            BinaryFunction::Add => {
                if self.lhs.is_zero() {
                    return *self.rhs.clone()
                }

                if self.rhs.is_zero() {
                    return *self.lhs.clone()
                }

                return Expr::binary_from(&self.lhs.get_cleaned(), &self.rhs.get_cleaned(), self.function)
            },
            BinaryFunction::Mul => {
                if self.lhs.is_zero() || self.rhs.is_zero() {
                    return Expr::from_integer(0)
                }

                if self.lhs.is_unity() {
                    return *self.rhs.clone()
                }

                if self.rhs.is_unity() {
                    return *self.lhs.clone()
                }

                return Expr::binary_from(&self.lhs.get_cleaned(), &self.rhs.get_cleaned(), self.function)
            },
            BinaryFunction::Div => {
                if self.lhs == self.rhs {
                    return Expr::from_integer(1)
                }

                return Expr::binary_from(&self.lhs.get_cleaned(), &self.rhs.get_cleaned(), self.function)
            }
            _ => return Expr::binary_from(&self.lhs.get_cleaned(), &self.rhs.get_cleaned(), self.function)
        }
    }

    fn get_derivative(&self, expr_key: &String, expr_map: &HashMap<String, Box<Expr>>) -> Expr {
        match self.function {
            BinaryFunction::Add => {
                return Expr::binary_from(&self.lhs.get_derivative(expr_key, expr_map),
                                         &self.rhs.get_derivative(expr_key, expr_map),
                                         BinaryFunction::Add)
            },
            // Implement for sub, mul and div
            _ => Expr::Binary(self.clone())
        }
    }
}

#[cfg(test)]
#[test]
fn test_symexpr_constants() {
    let one = Expr::from_integer(1);
    let zero = Expr::from_integer(0);
}

#[test]
fn test_basic_variable_mapping() {
    let mut expr_map: HashMap<String, Box<Expr>> = HashMap::new();
    let x_key = String::from("x");
    let x_val = Expr::from_real(5.);
    let x = Expr::from_key(&x_key);
    let one = Expr::from_integer(1);

    expr_map.insert(x_key, Box::new(x_val));

    let one_plus_x = one.add(&x);
    let one_minus_x = one.sub(&x);

    // 1 + x where x = 5 gives 6
    assert_eq!(one_plus_x.eval(&expr_map), Numeric::from_integer(6));
    assert_eq!(one_minus_x.eval(&expr_map), Numeric::from_integer(-4));
}

#[test]
fn test_basic_cleanup() {
    let x_key = String::from("x");
    let x = Expr::from_key(&x_key);
    let one = Expr::from_integer(1);
    let zero = Expr::from_integer(0);

    let one_plus_x = one.add(&x);
    assert_eq!(one_plus_x.get_cleaned(), one_plus_x);

    // 1 * (1 + x) = 1 + x
    let messy_expr_1 = one.mul(&one_plus_x);
    assert_eq!(messy_expr_1.get_cleaned(), one_plus_x);
    
    // 0 * (1 + x) = 0
    let messy_expr_2 = zero.mul(&one_plus_x);
    assert_eq!(messy_expr_2.get_cleaned(), zero);
    
    // 0 + (1 + x) = 1 + x
    let messy_expr_3 = zero.add(&one_plus_x);
    assert_eq!(messy_expr_3.get_cleaned(), one_plus_x);
    
    // (1 + x) / (1 + x) = 1
    let messy_expr_4 = one_plus_x.div(&one_plus_x);
    assert_eq!(messy_expr_4.get_cleaned(), one);
}

#[test]
fn test_power_derivative() {
    let mut expr_map: HashMap<String, Box<Expr>> = HashMap::new();

    let x_key = String::from("x");
    let x_var = Expr::from_key(&x_key);
    let x = Box::new(Expr::from_integer(5));
    let one = Expr::from_integer(1);

    expr_map.insert(x_key.clone(), x.clone());

    let y_key = String::from("y");
    let y_var = Expr::from_key(&y_key.clone());
    let y = one.add(&x_var);
    expr_map.insert(y_key.clone(), Box::new(y.clone()));

    let y_squared = y_var.pow(2);

    // Verify that variable dependency check uses the hash map correctly
    assert!(y.depends_on_variable(&x_key, &expr_map));
    assert!(y_squared.depends_on_variable(&x_key, &expr_map));

    // For (x = 5) & (y = 1 + x)
    // 1 + x = 6
    assert_eq!(y.eval(&expr_map), Numeric::from_integer(6));

    // d/dx -> (1 + x) = 1 
    assert_eq!(y.get_derivative(&x_key, &expr_map).eval(&expr_map), Numeric::from_integer(1));

    // (1 + x)^2 = 36
    assert_eq!(y_squared.eval(&expr_map), Numeric::from_integer(36));
    
    // d/dx -> (1 + x)^2 = 2(1 + x) = 12
    assert_eq!(y_squared.get_derivative(&x_key, &expr_map).eval(&expr_map), Numeric::from_integer(12));
}
