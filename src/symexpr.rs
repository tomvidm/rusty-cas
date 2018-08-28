use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, PartialEq)]
enum Expression {
    Constant(f64),
    IntegerConstant(i64),
    Variable(usize),
    UnaryExpr(UnaryExpression),
    BinaryExpr(BinaryExpression)
}

#[derive(Clone, PartialEq)]
enum UnaryFunction {
    Neg, Sin, Cos
}

#[derive(Clone, PartialEq)]
enum BinaryFunction {
    Add, Sub, Mul, Div, Pow
}

#[derive(Clone, PartialEq)]
pub struct UnaryExpression {
    function: UnaryFunction,
    argument: Box<Expression>
}

#[derive(Clone, PartialEq)]
pub struct BinaryExpression {
    function: BinaryFunction,
    lhs: Box<Expression>,
    rhs: Box<Expression>
}

impl Expression {
    fn calculate(&self, argument_list: &Vec<f64>) -> f64 {
        match self {
            Expression::Constant(val) => return *val,
            Expression::IntegerConstant(intval) => return *intval as f64,
            Expression::Variable(arg_index) => return argument_list[*arg_index],
            Expression::UnaryExpr(unary_expression) => return unary_expression.calculate(argument_list),
            Expression::BinaryExpr(binary_expression) => return binary_expression.calculate(argument_list)
        }
    }

    fn depends_on_variable(&self, variable: usize) -> bool {
        match self {
            Expression::Constant(val) => return false,
            Expression::IntegerConstant(intval) => return false,
            Expression::Variable(arg_index) => *arg_index == variable,
            Expression::UnaryExpr(unary_expression) => unary_expression.depends_on_variable(variable),
            Expression::BinaryExpr(binary_expression) => binary_expression.depends_on_variable(variable)
        }
    }

    fn from_float(val: f64) -> Expression {
        return Expression::Constant(val)
    }

    fn from_integer(intval: i64) -> Expression {
        return Expression::IntegerConstant(intval)
    }

    fn new_variable(index: usize) -> Expression {
        return Expression::Variable(index)
    }

    fn new_unary_expr(arg: Expression, function: UnaryFunction) -> Expression {
        return Expression::UnaryExpr(
            UnaryExpression{
                function: function,
                argument: Box::new(arg)
            }
        )
    }

    fn new_binary_expr(lhs: Expression, rhs: Expression, function: BinaryFunction) -> Expression {
        return Expression::BinaryExpr(
            BinaryExpression{
                function: function,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs)
            }
        )
    }

    fn get_derivative(&self, variable: usize) -> Expression {
        match self {
            Expression::Constant(val) => return Expression::Constant(0.),
            Expression::IntegerConstant(val) => return Expression::IntegerConstant(0),
            Expression::Variable(arg_index) => {
                if *arg_index == variable {
                    return Expression::Constant(1.)
                } else {
                    return Expression::Constant(0.)
                }
            },
            Expression::UnaryExpr(unary_expression) => {
                if unary_expression.depends_on_variable(variable) {
                    return unary_expression.argument.get_derivative(variable)
                } else {
                    return Expression::Constant(0.)
                }
            },
            Expression::BinaryExpr(binary_expression) => {
                if binary_expression.depends_on_variable(variable) {
                    return binary_expression.get_derivative(variable)
                } else {
                    return Expression::Constant(0.)
                }
            }
        }
    }
}

impl UnaryExpression {
    fn calculate(&self, argument_list: &Vec<f64>) -> f64 {
        match self.function {
            UnaryFunction::Neg => return -self.argument.calculate(argument_list),
            UnaryFunction::Sin => return self.argument.calculate(argument_list).sin(),
            UnaryFunction::Cos => return self.argument.calculate(argument_list).cos()
        }
    }

    fn depends_on_variable(&self, variable: usize) -> bool {
        self.argument.depends_on_variable(variable)
    }

    fn get_derivative(&self, variable: usize) -> Expression {
        match self.function {
            UnaryFunction::Neg => {
                return Expression::new_unary_expr(
                    self.argument.get_derivative(variable),
                    UnaryFunction::Neg
                )
            },
            UnaryFunction::Sin => {
                return Expression::new_binary_expr(
                    self.argument.get_derivative(variable),
                    Expression::new_unary_expr(
                        *self.argument.clone(),
                        UnaryFunction::Cos
                    ),
                    BinaryFunction::Mul
                )
            },
            UnaryFunction::Cos => {
                return Expression::new_binary_expr(
                    Expression::new_unary_expr(
                        self.argument.get_derivative(variable),
                        UnaryFunction::Neg
                    ),
                    Expression::new_unary_expr(
                        *self.argument.clone(),
                        UnaryFunction::Sin
                    ),
                    BinaryFunction::Mul
                )
            }
        }
    }
}

impl BinaryExpression {
    fn calculate(&self, argument_list: &Vec<f64>) -> f64 {
        let lhs = self.lhs.calculate(argument_list);
        let rhs = self.rhs.calculate(argument_list);
        match self.function {
            BinaryFunction::Add => return lhs + rhs,
            BinaryFunction::Sub => return lhs - rhs,
            BinaryFunction::Mul => return lhs * rhs,
            BinaryFunction::Div => return lhs / rhs,
            BinaryFunction::Pow => return lhs.powf(rhs)
        }
    }

    fn depends_on_variable(&self, variable: usize) -> bool {
        return self.lhs.depends_on_variable(variable) || self.rhs.depends_on_variable(variable)
    }

    fn get_derivative(&self, variable: usize) -> Expression {
        match self.function {
            // Simple linera addition
            BinaryFunction::Add => {
                return Expression::new_binary_expr(
                    self.lhs.get_derivative(variable),
                    self.rhs.get_derivative(variable),
                    BinaryFunction::Add
                )
            },
            // Simple linera subtraction
            BinaryFunction::Sub => {
                return Expression::new_binary_expr(
                    self.lhs.get_derivative(variable),
                    self.rhs.get_derivative(variable),
                    BinaryFunction::Sub
                )
            },
            // Product rule
            BinaryFunction::Mul => {
                // f'(x)g(x) + f(x)g'(x)
                return Expression::new_binary_expr(
                    // f'(x)g(x)
                    Expression::new_binary_expr(
                        *self.lhs.clone(),
                        self.rhs.get_derivative(variable),
                        BinaryFunction::Mul
                    ),
                    // f(x)g'(x)
                    Expression::new_binary_expr(
                        self.lhs.get_derivative(variable),
                        *self.rhs.clone(),
                        BinaryFunction::Mul
                    ),
                    BinaryFunction::Add
                )
            }
            // Quotient rule
            BinaryFunction::Div => {
                // [g'(x)h(x) - g(x)h'(x)] / h(x)^2
                return Expression::new_binary_expr(
                    // g'(x)h(x) - g(x)h'(x)
                    Expression::new_binary_expr(
                        // g'(x)h(x)
                        Expression::new_binary_expr(
                            self.lhs.get_derivative(variable),
                            *self.rhs.clone(),
                            BinaryFunction::Mul
                        ),
                        // g(x)h'(x)
                        Expression::new_binary_expr(
                            *self.lhs.clone(),
                            self.rhs.get_derivative(variable),
                            BinaryFunction::Mul
                        ),
                        BinaryFunction::Sub
                    ),
                    // h(x)^2
                    Expression::new_binary_expr(
                        *self.rhs.clone(),
                        *self.rhs.clone(),
                        BinaryFunction::Mul
                    ),
                    BinaryFunction::Div
                )
            },
            BinaryFunction::Pow => {
                println!("Pow function for expressions is not yet implemented...");
                return Expression::Constant(0.)
            }
        }
    }
}

// ==================
// Operator overloads
// ==================

impl Expression {
    fn add(&self, other: &Expression) -> Expression {
        return Expression::new_binary_expr(self.clone(), other.clone(), BinaryFunction::Add)
    }

    fn sub(&self, other: &Expression) -> Expression {
        return Expression::new_binary_expr(self.clone(), other.clone(), BinaryFunction::Sub)
    }

    fn mul(&self, other: &Expression) -> Expression {
        return Expression::new_binary_expr(self.clone(), other.clone(), BinaryFunction::Mul)
    }

    fn div(&self, other: &Expression) -> Expression {
        return Expression::new_binary_expr(self.clone(), other.clone(), BinaryFunction::Div)
    }
}

#[cfg(test)]
#[test]
fn test_symexpr() {
    let a = Expression::from_float(2.);
    let b = Expression::new_variable(0);
    let c = Expression::new_variable(1);
    let arglist = vec![3., 4.];

    assert_eq!(a.mul(&b.add(&c)).calculate(&arglist), 14.);
}

#[test]
fn test_derivative() {
    let arglist = vec![5.];
    let a = Expression::Constant(2.);
    let x = Expression::Variable(0);
    let c = Expression::Constant(5.);
    let expr = a.mul(&x.mul(&x)).add(&c.div(&x));
    assert_eq!(expr.calculate(&arglist), 51.);
    assert_eq!(expr.get_derivative(0).calculate(&arglist), 19.8);
}