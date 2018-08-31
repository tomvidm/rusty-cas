use numeric::{Numeric, IntegerType};
use std::io;
use std::char;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Term(TermToken),
    Operator(Operator)
}

impl Token {
    fn is_term(&self) -> bool {
        match self {
            Token::Term(_term) => return true,
            _ => return false
        }
    }

    fn is_numeric(&self) -> bool {
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

    fn is_variable(&self) -> bool {
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
}

#[derive(Clone, PartialEq, Debug)]
pub struct Operator {
    precedence: i64,
    op: OperatorType
}

const LeftP: Operator = Operator{precedence: 1, op: OperatorType::LeftP};
const RightP: Operator = Operator{precedence: 1, op: OperatorType::RightP};
const Add: Operator = Operator{precedence: 2, op: OperatorType::Add};
const Mul: Operator = Operator{precedence: 4, op: OperatorType::Mul};

#[derive(Clone, PartialEq, Debug)]
pub enum TermToken {
    Number(Numeric),
    VariableKey(String)
}

#[derive(Clone, PartialEq, Debug)]
pub enum OperatorType {
    LeftP,
    RightP,
    Add,
    Mul
}

type Tokens = Vec<Token>;

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

fn tokenize_string(string: &String) -> Tokens {
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

fn infix_to_postfix(tokens: &Tokens) -> Tokens {
    let mut infix: Vec<Token> = Vec::new();

    for token in tokens.iter() {

    }

    return tokens.clone()
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
}

#[test]
fn test_infix_to_postfix() {
    use self::Token::{Term, Operator};
    use self::TermToken::{Number, VariableKey};

    let expr1: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2)))
    ];

    let expr1_postfix: Vec<Token> = vec![
        Term(TermToken::Number(Numeric::from_integer(2)))
    ];
    
    assert_eq!(infix_to_postfix(&expr1), expr1_postfix);

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
}