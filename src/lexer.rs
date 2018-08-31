use numeric::{Numeric, IntegerType};
use std::io;
use std::char;
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Token {
    Term(TermToken),
    Operator(Operator)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Term(term) => {
                match term {
                    TermToken::Number(number) => write!(f, "{:?}", *number),
                    TermToken::VariableKey(key) => write!(f, "{:?}", *key)
                }
            },
            Token::Operator(op) => {
                match op.op {
                    OperatorType::Add => write!(f, "+"),
                    OperatorType::Mul => write!(f, "*"),
                    OperatorType::LeftP => write!(f, "("),
                    OperatorType::RightP => write!(f, ")"),
                    OperatorType::Assignment => write!(f, "=")
                }
            }
        }
    }
}

impl Token {
    pub fn is_term(&self) -> bool {
        match self {
            Token::Term(_term) => return true,
            _ => return false
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_op) => return true,
            _ => return false
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Token::Term(term) => {
                match term {
                    TermToken::Number(_numeric) => return true,
                    _ => return false
                }
            },
            _ => return false
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            Token::Term(term) => {
                match term {
                    TermToken::VariableKey(_key) => return true,
                    _ => return false
                }
            },
            _ => return false
        }
    }

    fn is_left_parenthesis(&self) -> bool {
        match self {
            Token::Operator(op) => *op == LeftP,
            _ => return false
        }
    }

    fn is_right_parenthesis(&self) -> bool {
        match self {
            Token::Operator(op) => *op == RightP,
            _ => return false
        }
    }

    pub fn is_assignment(&self) -> bool {
        match self {
            Token::Operator(op) => *op == Assignment,
            _ => return false
        }
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct Operator {
    precedence: i64,
    pub op: OperatorType
}

const LeftP: Operator = Operator{precedence: 1, op: OperatorType::LeftP};
const RightP: Operator = Operator{precedence: 1, op: OperatorType::RightP};
const Add: Operator = Operator{precedence: 2, op: OperatorType::Add};
const Mul: Operator = Operator{precedence: 4, op: OperatorType::Mul};
const Assignment: Operator = Operator{precedence: 4, op: OperatorType::Assignment};

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum TermToken {
    Number(Numeric),
    VariableKey(String)
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum OperatorType {
    LeftP,
    RightP,
    Add,
    Mul,
    Assignment
}

pub type Tokens = Vec<Token>;

fn tokenize_number(slice: &str) -> Token {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    for ch in slice.chars() {
    }
        return Operator(Add)
}

fn cocatenate_numeric_tokens(tokens: &[Token]) -> Token {
    let mut result = Numeric::from_integer(0);
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Term(term) => {
                match term {
                    TermToken::Number(numeric) => result = Numeric::from_integer(10) * result + *numeric,
                    _ => break
                }
            }
            _ => break
        }
    }

    return Token::Term(TermToken::Number(result));
}

fn get_length_of_numeric_sequence(tokens: &[Token]) -> usize {
    for (i, tok) in tokens.iter().enumerate() {
        if !tok.is_numeric() {
            return i
        }
    }
    return tokens.len()
}

pub fn tokenize_string(string: &String) -> Tokens {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    let mut tokens: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();
    let mut temp_number = Numeric::from_integer(0);
    for (i, ch) in string.chars().enumerate() {
        match ch {
            ' ' => continue,
            '(' => tokens.push(Operator(LeftP)),
            ')' => tokens.push(Operator(RightP)),
            '+' => tokens.push(Operator(Add)),
            '*' => tokens.push(Operator(Mul)),
            '=' => tokens.push(Operator(Assignment)),
            '0'...'9' => tokens.push(Term(TermToken::Number(Numeric::from_integer(ch.to_digit(10).unwrap() as IntegerType)))),
            'A'...'z' => tokens.push(Term(TermToken::VariableKey(ch.to_string()))),
            _ => continue
        }
    }

    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].is_numeric() {
            let index_of_next_non_numeric = i + get_length_of_numeric_sequence(&tokens[i..]);
            result.push(cocatenate_numeric_tokens(&tokens[i..]));
            i = index_of_next_non_numeric;
            if i < tokens.len() && tokens[i].is_left_parenthesis() {
                result.push(Operator(Mul));
            }
        } else if tokens[i].is_variable() {
            result.push(tokens[i].clone());
            if i < tokens.len() - 1 && tokens[i + 1].is_left_parenthesis() {
                result.push(Operator(Mul));
            }    
            i += 1;
        } else {
            result.push(tokens[i].clone());
            i += 1;
        }
    }
    return result;
}

pub fn infix_to_postfix(tokens: &Tokens) -> Tokens {
    let mut postfix: Vec<Token> = Vec::new();
    let mut opstack: Vec<Token> = Vec::new();
    // For each token
    for (i, token) in tokens.iter().enumerate() {
        // If a number or variable
        if token.is_term() {
            postfix.push(token.clone());
        } 
        // If operator or parenthesis
        else if token.is_operator() {
            // If left parenthesis
            if token.is_left_parenthesis() {
                opstack.push(token.clone());
            } 
            // If right parenthesis
            else if token.is_right_parenthesis() {
                // Pop top of opstack if not empty
                while let Some(op_from_stack) = opstack.pop() {
                    // Pop operators onto output until a left parenthesis
                    if op_from_stack.is_left_parenthesis() {
                        break;
                    } else {
                        postfix.push(op_from_stack.clone());
                    }
                }
            } 
            // Else
            else {
                // Pop top of opstack if not empty
                while let Some(op_from_stack) = opstack.pop() {
                    // If top operator has higher precedence than current token
                    if op_from_stack.is_left_parenthesis() {
                        opstack.push(op_from_stack);
                        break;
                    }
                    
                    if op_from_stack > *token {
                        /// Push top operator to output
                        postfix.push(op_from_stack.clone());
                    } else {
                        opstack.push(op_from_stack);
                        break;
                    }
                }
                // Push token to output
                opstack.push(token.clone());
            }
        }
    }

    while let Some(remaining_op) = opstack.pop() {
        postfix.push(remaining_op.clone());
    }

    return postfix;
}

pub fn string_to_postfix(string: &String) -> Tokens {
    infix_to_postfix(&tokenize_string(string))
}

#[cfg(test)]
#[test]
fn test_operator_precedence() {
    use self::Token::{Term, Operator};
    assert!(LeftP < Add);
    assert!(Operator(LeftP) < Operator(Add));
    assert!(Operator(Add) < Operator(Mul));
    assert!(!Operator(LeftP).is_numeric());
}

#[cfg(test)]
#[test]
fn test_concatenation() {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    let tokenized: Vec<Token> = vec![
        Operator(Add),
        Term(TermToken::Number(Numeric::from_integer(2))), 
        Term(TermToken::Number(Numeric::from_integer(3))), 
        Term(TermToken::Number(Numeric::from_integer(5))), 
        Operator(Add), 
        Term(VariableKey("x".to_string()))
    ];

    let index_of_next_non_numeric = 1 + get_length_of_numeric_sequence(&tokenized[1..]);
    assert_eq!(index_of_next_non_numeric, 4);
    let result = cocatenate_numeric_tokens(&tokenized[1..]);
    assert_eq!(result, Term(TermToken::Number(Numeric::from_integer(235))));
}

#[test]
fn test_tokenizer() {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    // Check that space and trailing ; does are both ignored
    let expr1_variation1 = String::from(" 2");
    let expr1_variation2 = String::from("2 ;");
    let expr1_tokenized: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2)))
    ];

    assert_eq!(tokenize_string(&expr1_variation1), expr1_tokenized);
    assert_eq!(tokenize_string(&expr1_variation2), expr1_tokenized);

    // Check that integers and variables and operators are processed
    let expr2_variation1 = String::from("20 + x;");
    let expr2_variation2 = String::from("20 + x");
    let expr2_tokenized: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(20))), 
        Operator(Add), 
        Term(VariableKey("x".to_string()))
    ];

    assert_eq!(tokenize_string(&expr2_variation1), expr2_tokenized);
    assert_eq!(tokenize_string(&expr2_variation2), expr2_tokenized);

    // Check that numbers followed by left parenthesis has multiplication added explicitly
    let expr3_variation1 = String::from("32 * (x + y);");
    let expr3_variation2 = String::from("32(x + y)");
    let expr3_variation3 = String::from("32 (x+ y);");

    let expr3_tokenized: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(32))),
        Operator(Mul),
        Operator(LeftP),
        Term(TermToken::VariableKey("x".to_string())),
        Operator(Add),
        Term(TermToken::VariableKey("y".to_string())),
        Operator(RightP)
    ];

    assert_eq!(tokenize_string(&expr3_variation1), expr3_tokenized);
    assert_eq!(tokenize_string(&expr3_variation2), expr3_tokenized);
    assert_eq!(tokenize_string(&expr3_variation3), expr3_tokenized);

    // Check that variables followed by left parenthesis has multipication added explicitly
    let expr4_variation1 = String::from("x *( 1+x)");
    let expr4_variation2 = String::from("x ( 1+x)");
    let expr4_tokenized = vec![
        Term(TermToken::VariableKey("x".to_string())),
        Operator(Mul),
        Operator(LeftP),
        Term(TermToken::Number(Numeric::from_integer(1))),
        Operator(Add),
        Term(TermToken::VariableKey("x".to_string())),
        Operator(RightP)
    ];

    assert_eq!(tokenize_string(&expr4_variation1), expr4_tokenized);
    assert_eq!(tokenize_string(&expr4_variation2), expr4_tokenized);

    let expr4 = String::from("2 + 3(1 +x)");
    let expr4_tokenized: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2))), 
        Operator(Add),
        Term(TermToken::Number(Numeric::from_integer(3))), 
        Operator(Mul),
        Operator(LeftP),
        Term(TermToken::Number(Numeric::from_integer(1))),
        Operator(Add),
        Term(VariableKey("x".to_string())),
        Operator(RightP)
    ];

    assert_eq!(tokenize_string(&expr4), expr4_tokenized);
}

#[test]
fn test_infix_to_postfix() {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    // 2
    let expr1: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2)))
    ];

    let expr1_postfix: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2)))
    ];
    
    assert_eq!(infix_to_postfix(&expr1), expr1_postfix);

    // 20 + x
    let expr2: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(20))), 
        Operator(Add), 
        Term(VariableKey("x".to_string()))
    ];

    let expr2_postfix: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(20))), 
        Term(VariableKey("x".to_string())),
        Operator(Add)
    ];

    assert_eq!(infix_to_postfix(&expr2), expr2_postfix);

    // a((123) + x)

    let expr3: Vec<Token> = vec![
        Term(VariableKey("a".to_string())),
        Operator(Mul),
        Operator(LeftP),
        Operator(LeftP),
        Term(TermToken::Number(Numeric::from_integer(123))), 
        Operator(RightP),
        Operator(Add),
        Term(VariableKey("x".to_string())),
        Operator(RightP)
    ];

    let expr3_postfix: Vec<Token> = vec![
        Term(VariableKey("a".to_string())),
        Term(TermToken::Number(Numeric::from_integer(123))), 
        Term(VariableKey("x".to_string())),
        Operator(Add),
        Operator(Mul)
    ];

    assert_eq!(infix_to_postfix(&expr3), expr3_postfix);

    let expr4_tokenized: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2))), 
        Operator(Add),
        Term(TermToken::Number(Numeric::from_integer(3))), 
        Operator(Mul),
        Operator(LeftP),
        Term(TermToken::Number(Numeric::from_integer(1))),
        Operator(Add),
        Term(VariableKey("x".to_string())),
        Operator(RightP)
    ];

    let expr4_postfix: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2))), 
        Term(TermToken::Number(Numeric::from_integer(3))), 
        Term(TermToken::Number(Numeric::from_integer(1))),
        Term(VariableKey("x".to_string())),
        Operator(Add),
        Operator(Mul),
        Operator(Add)
    ];

    assert_eq!(infix_to_postfix(&expr4_tokenized), expr4_postfix);
}