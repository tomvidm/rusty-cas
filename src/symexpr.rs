#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Constant(f64),
    IntegerConstant(i64),
    Variable(usize),
    UnaryExpr(UnaryExpression),
    BinaryExpr(BinaryExpression),
    Error(String)
}

#[derive(Clone, PartialEq, Debug)]
enum UnaryFunction {
    Neg, Sin, Cos, Exp, Sqrt
}

#[derive(Clone, PartialEq, Debug)]
enum BinaryFunction {
    Add, Sub, Mul, Div, Pow
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryExpression {
    function: UnaryFunction,
    argument: Box<Expression>
}

#[derive(Clone, PartialEq, Debug)]
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
            Expression::BinaryExpr(binary_expression) => return binary_expression.calculate(argument_list),
            Expression::Error(error_string) => 0. //Maybe some NaN value
        }
    }

    fn depends_on_variable(&self, variable: usize) -> bool {
        match self {
            Expression::Constant(val) => return false,
            Expression::IntegerConstant(intval) => return false,
            Expression::Variable(arg_index) => *arg_index == variable,
            Expression::UnaryExpr(unary_expression) => unary_expression.depends_on_variable(variable),
            Expression::BinaryExpr(binary_expression) => binary_expression.depends_on_variable(variable),
            Expression::Error(error_string) => false
        }
    }

    fn depends_on_any_variable(&self) -> bool {
        match self {
            Expression::Constant(val) => return true,
            Expression::IntegerConstant(intval) => return true,
            Expression::Variable(var) => return false,
            Expression::UnaryExpr(unary_expr) => unary_expr.depends_on_any_variable(),
            Expression::BinaryExpr(binary_expr) => binary_expr.depends_on_any_variable(),
            Expression::Error(error_string) => return false
        }
    }

    fn is_unity(&self) -> bool {
        match self {
            Expression::Constant(val) => return *val == 1.,
            Expression::IntegerConstant(intval) => return *intval == 1,
            _ => return false
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Expression::Constant(val) => return *val == 0.,
            Expression::IntegerConstant(intval) => return *intval == 0,
            _ => return false
        }
    }

    fn one() -> Expression {
        return Expression::IntegerConstant(1)
    }

    fn zero() -> Expression {
        return Expression::IntegerConstant(0)
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
        ).get_simplified()
    }

    fn new_binary_expr(lhs: Expression, rhs: Expression, function: BinaryFunction) -> Expression {
        return Expression::BinaryExpr(
            BinaryExpression{
                function: function,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs)
            }
        ).get_simplified()
    }

    fn new_error(error_string: String) -> Expression {
        return Expression::Error(error_string)
    }

    fn get_derivative(&self, variable: usize) -> Expression {
        match self {
            Expression::Constant(val) => return Expression::Constant(0.),
            Expression::IntegerConstant(intval) => return Expression::IntegerConstant(0),
            Expression::Variable(arg_index) => {
                if *arg_index == variable {
                    return Expression::Constant(1.)
                } else {
                    return Expression::Constant(0.)
                }
            },
            Expression::UnaryExpr(unary_expression) => {
                if unary_expression.depends_on_variable(variable) {
                    return unary_expression.get_derivative(variable)
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
            },
            Expression::Error(error_string) => return self.clone()
        }
    }

    fn get_simplified(&self) -> Expression {
        match self {
            Expression::Constant(a) => self.clone(),
            Expression::IntegerConstant(a) => self.clone(),
            Expression::Variable(a) => self.clone(),
            Expression::UnaryExpr(unary_expr) => unary_expr.get_simplified(),
            Expression::BinaryExpr(binary_expr) => binary_expr.get_simplified(),
            Expression::Error(error_string) => return self.clone()
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::Constant(val) => return val.to_string(),
            Expression::IntegerConstant(intval) => return intval.to_string(),
            Expression::Variable(arg_index) => return format!("[{}]", arg_index.to_string()),
            Expression::UnaryExpr(unary_expression) => return unary_expression.to_string(),
            Expression::BinaryExpr(binary_expression) => return binary_expression.to_string(),
            Expression::Error(error_string) => return error_string.clone()
        }
    }
}

impl UnaryExpression {
    fn calculate(&self, argument_list: &Vec<f64>) -> f64 {
        let argument_value = self.argument.calculate(argument_list);
        match self.function {
            UnaryFunction::Neg => return -argument_value,
            UnaryFunction::Sin => return argument_value.sin(),
            UnaryFunction::Cos => return argument_value.cos(),
            UnaryFunction::Exp => return argument_value.exp(),
            UnaryFunction::Sqrt => return argument_value.sqrt()
        }
    }

    fn depends_on_variable(&self, variable: usize) -> bool {
        self.argument.depends_on_variable(variable)
    }

    fn depends_on_any_variable(&self) -> bool {
        return self.argument.depends_on_any_variable();
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
            },
            UnaryFunction::Exp => {
                return Expression::new_binary_expr(
                    self.argument.get_derivative(variable),
                    Expression::UnaryExpr(self.clone()),
                    BinaryFunction::Mul
                )
            }
            UnaryFunction::Sqrt => {
                return Expression::new_binary_expr(
                    self.argument.get_derivative(variable),
                    Expression::new_binary_expr(
                        Expression::from_float(2.),
                        Expression::UnaryExpr(self.clone()),
                        BinaryFunction::Mul
                    ),
                    BinaryFunction::Div
                )
            }
        }
    }

    fn get_simplified(&self) -> Expression {
        Expression::UnaryExpr(self.clone())
    }

    fn to_string(&self) -> String {
        let arg_string = self.argument.to_string();
        match self.function {
            UnaryFunction::Neg => format!("(-{})", arg_string),
            UnaryFunction::Sin => format!("sin({})", arg_string),
            UnaryFunction::Cos => format!("cos({})", arg_string),
            UnaryFunction::Exp => format!("exp({})", arg_string),
            UnaryFunction::Sqrt => format!("sqrt({})", arg_string)
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

    fn depends_on_any_variable(&self) -> bool {
        return self.lhs.depends_on_any_variable() ||
               self.rhs.depends_on_any_variable()
    }

    fn get_derivative(&self, variable: usize) -> Expression {
        match self.function {
            // Simple linear addition
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
            },
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
                        Expression::from_integer(2),
                        BinaryFunction::Pow
                    ),
                    BinaryFunction::Div
                )
            },
            // This one only accounts for pow(x,y) where y >= 0
            BinaryFunction::Pow => {
                // g'(x) * f'(X)^(g(x))
                return Expression::new_binary_expr(
                    // g'(x)
                    self.rhs.get_derivative(variable),
                    // f'(x) ^ g(x)
                    Expression::new_binary_expr(
                        self.lhs.get_derivative(variable),
                        self.rhs.clone().sub(&Expression::from_integer(1)),
                        BinaryFunction::Pow
                    ),
                    BinaryFunction::Mul
                )
            }
        }
    }

    // This method returns simplified expression, based on simple rules.
    // More sophisticated methods should be handled by a solver. This is just
    // to clean up expressions on the go.
    //   0 + a = a
    //   0 * a = 0
    //   1 * a = a
    //   a / a = 1
    fn get_simplified(&self) -> Expression {
        match self.function {
            BinaryFunction::Add => {
                if self.lhs.is_zero() {
                    return *self.rhs.clone()
                }

                if self.rhs.is_zero() {
                    return *self.lhs.clone()
                }

                return Expression::BinaryExpr(self.clone())
            },

            BinaryFunction::Mul => {
                if self.lhs.is_zero() || self.rhs.is_zero() {
                    return Expression::from_integer(0)
                }

                if self.lhs.is_unity() {
                    return *self.rhs.clone()
                }

                if self.rhs.is_unity() {
                    return *self.lhs.clone()
                }

                return Expression::BinaryExpr(self.clone())
            },

            BinaryFunction::Div => {
                if self.lhs == self.rhs {
                    return Expression::from_integer(1)
                }

                return Expression::BinaryExpr(self.clone())
            }
            _ => return Expression::BinaryExpr(self.clone())
        }
    }

    fn to_string(&self) -> String {
        let lhs_string = self.lhs.to_string();
        let rhs_string = self.rhs.to_string();
        match self.function {
            BinaryFunction::Add => return format!("({} + {})", lhs_string, rhs_string),
            BinaryFunction::Sub => return format!("({} - {})", lhs_string, rhs_string),
            BinaryFunction::Mul => return format!("({} * {})", lhs_string, rhs_string),
            BinaryFunction::Div => return format!("({} / {})", lhs_string, rhs_string),
            BinaryFunction::Pow => return format!("({} ^ {})", lhs_string, rhs_string)
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

    fn pow(&self, other: &Expression) -> Expression {
        return Expression::new_binary_expr(self.clone(), other.clone(), BinaryFunction::Pow)
    }

    fn sin(&self, other: &Expression) -> Expression {
        return Expression::new_unary_expr(self.clone(), UnaryFunction::Sin)   
    }

    fn cos(&self, other: &Expression) -> Expression {
        return Expression::new_unary_expr(self.clone(), UnaryFunction::Cos)   
    }

    fn exp(&self) -> Expression {
        return Expression::new_unary_expr(self.clone(), UnaryFunction::Exp)
    }

    fn sqrt(&self) -> Expression {
        return Expression::new_unary_expr(self.clone(), UnaryFunction::Sqrt)
    }
}

#[cfg(test)]
#[test]
fn test_symexpr() {
    let a = Expression::from_float(2.);
    let b = Expression::new_variable(0);
    let c = Expression::new_variable(1);
    let arglist = vec![3., 4.];
    let d = Expression::from_float(2.);

    assert_eq!(a.mul(&b.add(&c)).calculate(&arglist), 14.);
    assert_eq!(a, d);
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

    let exparg = a.add(&x);
    let powexpr = exparg.exp();

    assert_eq!(exparg.get_derivative(0).calculate(&arglist), 1.);
    assert_eq!(powexpr.calculate(&arglist), 1096.6331584284585);
    assert_eq!(powexpr.get_derivative(0).calculate(&arglist), 1096.6331584284585);

    let arglist2 = vec![0.25];
    let sqrt_expr = Expression::Variable(0).sqrt();
    assert_eq!(sqrt_expr.get_derivative(0).calculate(&arglist2), 1.);
}

#[test]
fn test_simplification() {
    let zero = Expression::zero();
    let one = Expression::one();
    let x = Expression::Variable(0);

    let expr = zero.add(&one.sin(&x));

    println!("{:?}", expr);

    assert_eq!(expr, Expression::UnaryExpr(
                    UnaryExpression { 
                        function: UnaryFunction::Sin, 
                        argument: Box::new(Expression::one())
                    }));
}