use numeric::{Numeric, IntegerType};
use std::fmt;

#[derive(PartialEq)]
pub enum Token {
    Term(Term),
    Operator(Operator)
}

impl Token {
    pub fn new_number(number: Numeric) -> Token {
        Token::Term(
            Term::Number(number)
        )
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Term(term) => {
                write!(f, "{:?}", term)
            },
            Token::Operator(operator) => {
                write!(f, "{:?}", operator)
            }
        }       
    }
}

#[derive(PartialEq)]
pub enum Term {
    Number(Numeric),
    Variable(i64)
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Number(numeric) => write!(f, "{:?}", *numeric),
            Term::Variable(key) => write!(f, "[{}]", key)
        }
    }
}

#[derive(PartialEq)]
enum Associativity {
    Left,
    Right,
    Both,
    Non
}

#[derive(PartialEq)]
enum OperatorType {
    Add,
    Mul,
    LeftP,
    RightP,
}

#[derive(PartialEq)]
pub struct Operator {
    precedence: i64,
    associativity: Associativity,
    operator_type: OperatorType
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.operator_type {
            OperatorType::Add => write!(f, "+"),
            OperatorType::Mul => write!(f, "*"),
            OperatorType::LeftP => write!(f, "("),
            OperatorType::RightP => write!(f, ")"),
        }
    }
}

impl Operator {
    pub fn add() -> Token {
        Token::Operator(
            Operator {
                precedence: 2,
                associativity: Associativity::Both,
                operator_type: OperatorType::Add
            }
        )
    }

    pub fn mul() -> Token {
        Token::Operator(
            Operator {
                precedence: 4,
                associativity: Associativity::Both,
                operator_type: OperatorType::Mul
            }
        )
    }

    pub fn left_parenthesis() -> Token {
        Token::Operator(
            Operator {
                precedence: 1,
                associativity: Associativity::Both,
                operator_type: OperatorType::LeftP
            }
        )
    }

    pub fn right_parenthesis() -> Token {
        Token::Operator(
            Operator {
                precedence: 1,
                associativity: Associativity::Both,
                operator_type: OperatorType::RightP
            }
        )
    }
}