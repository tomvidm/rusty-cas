use std::collections::{HashMap};
use numeric::{Numeric};
use symexpr_rc::{Expr};
use std::rc::Rc;

struct Engine {
    variable_map: HashMap<String, usize>,
    variable_list: Vec<Numeric>,
    expr_map: HashMap<String, usize>,
    expr_list: Vec<Rc<Expr>>
}

impl Engine {
    fn new() -> Engine {
        Engine {
            variable_map: HashMap::new(),
            variable_list: Vec::new(),
            expr_map: HashMap::new(),
            expr_list: Vec::new()
        }
    }

    fn eval_expr_with(&self,key: &String, values: &Vec<Numeric>) -> Option<Numeric> {
        match self.get_expression(key) {
            Some(expr) => return Some(expr.eval(&values)),
            None => return None
        }
    }

    fn eval_expr(&self, key: &String) -> Option<Numeric> {
        match self.get_expression(key) {
            Some(expr) => return Some(expr.eval(&self.variable_list)),
            None => return None
        }
    }

    fn get_variable(&self, key: &String) -> Option<Numeric> {
        match self.variable_map.get(key) {
            Some(index) => Some(self.variable_list[*index]),
            None => None
        }
    }

    fn get_index_of_variable(&self, key: &String) -> Option<(usize)> {
        match self.variable_map.get(key) {
            Some(index) => Some(*index),
            None => None
        }
    }

    fn assign_variable(&mut self, key: &String, val: Numeric) -> usize {
        match self.get_index_of_variable(key) {
            Some(index) => {
                self.variable_list[index] = val;
                return index
            },
            None => {
                let new_index = self.variable_list.len();
                self.variable_map.insert(key.clone(), new_index);
                self.variable_list.push(val);
                return new_index
            }
        }
    }

    fn get_expression(&self, key: &String) -> Option<Rc<Expr>> {
        match self.expr_map.get(key) {
            Some(index) => Some(Rc::clone(&self.expr_list[*index])),
            None => None
        }
    }

    fn get_index_of_expression(&self, key: &String) -> Option<usize> {
        match self.expr_map.get(key) {
            Some(index) => Some(*index),
            None => None
        }
    }

    fn assign_expression(&mut self, key: &String, expr: &Rc<Expr>) -> usize {
        match self.get_index_of_expression(key) {
            Some(index) => {
                self.expr_list[index] = Rc::clone(expr);
                return index
            },
            None => {
                let new_index = self.expr_list.len();
                self.expr_map.insert(key.clone(), new_index);
                self.expr_list.push(Rc::clone(expr));
                return new_index
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_symengine_assignment_and_evaluation() {
    let mut engine = Engine::new();
    let key = String::from("x");
    assert_eq!(engine.get_variable(&"x".to_string()), None);
    let x_index = engine.assign_variable(&"x".to_string(), Numeric::from_integer(0));
    assert_eq!(engine.get_index_of_variable(&"x".to_string()), Some(x_index));

    assert_eq!(engine.get_expression(&"f".to_string()), None);
    let f_index = engine.assign_expression(&"f".to_string(), &Expr::from_key(x_index).clone_to_heap());
    assert_eq!(engine.get_index_of_expression(&"f".to_string()), Some(f_index));
    assert_eq!(engine.eval_expr(&"f".to_string()), Some(Numeric::from_integer(0)));
    let custom_values: Vec<Numeric> = vec![Numeric::from_real(2.)];
    assert_eq!(engine.eval_expr_with(&"f".to_string(), &custom_values), Some(Numeric::from_real(2.)));
}