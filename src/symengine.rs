use std::collections::{HashMap};

type Expression = f64;

struct Engine {
    expression_map: HashMap<String, Expression>,
    expression_list: Vec<Expression>
}