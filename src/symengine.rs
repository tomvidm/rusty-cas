use std::collections::{HashMap};
use numeric::{Numeric};
use symexpr::{Expr};

struct Engine {
    expression_map: HashMap<String, Box<Expr>>,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            expression_map: HashMap::new()
        }
    }

    fn assign_variable(&mut self, key: &String, val: &Expr) {
        self.expression_map.insert(key.clone(), Box::new(val.clone()));
    }

    fn evaluate_variable(&mut self, key: &String) -> Numeric {
        match self.expression_map.get(key) {
            Some(boxed_expr) => {
                return boxed_expr.eval(&self.expression_map);
            },
            None => return Numeric::zero()
        }
    }
}

#[cfg(test)]
#[test]
fn test_symengine() {
    let mut engine = Engine::new();

    let x_val = Expr::from_integer(5);
    let x_key = String::from("x");

    engine.assign_variable(&x_key, &x_val);

    assert_eq!(engine.evaluate_variable(&x_key), Numeric::from_integer(5));
}