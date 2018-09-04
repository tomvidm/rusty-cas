use token::{Token, Term, Operator};
use numeric::{Numeric};

type Tokens = Vec<Token>;

// ===================
// String tokenization
// ===================

fn tokenize_string(string: &str) -> Tokens {
    Tokens::new()
}

// ===========================
// Infix to postfix conversion
// ===========================

#[derive(PartialEq, Debug)]
enum InfixConversionResult {
    Tokens(Tokens),
    InfixConversionError(InfixConversionError)
}

#[derive(PartialEq, Debug)]
enum InfixConversionError {
    Something,
    MismatchedParenthesis
}

fn infix_to_postfix(tokens: &Tokens) -> InfixConversionResult {
    InfixConversionResult::InfixConversionError(
        InfixConversionError::Something
    )
}

#[cfg(test)]
#[test]
fn test_infix_to_postfix_conversion() {
    let infix1: Tokens = vec![
        Token::new_number(Numeric::from_integer(1)),
        Operator::add(),
        Operator::left_parenthesis(),
        Operator::left_parenthesis(),
        Token::new_number(Numeric::from_integer(2)),
        Operator::mul(),
        Token::new_number(Numeric::from_integer(3)),
        Operator::right_parenthesis(),
        Operator::add(),
        Token::new_number(Numeric::from_integer(4)),
        Operator::right_parenthesis()
    ];

    let infix1_mismatched_parenthesis: Tokens = vec![
        Token::new_number(Numeric::from_integer(1)),
        Operator::add(),
        Operator::left_parenthesis(),
        Token::new_number(Numeric::from_integer(2)),
        Operator::left_parenthesis(),
        Operator::mul(),
        Token::new_number(Numeric::from_integer(3)),
        Operator::right_parenthesis(),
        Operator::add(),
        Token::new_number(Numeric::from_integer(4)),
        Operator::right_parenthesis()
    ];

    let postfix1: Tokens = vec![
        Token::new_number(Numeric::from_integer(1)),
        Token::new_number(Numeric::from_integer(2)),
        Token::new_number(Numeric::from_integer(3)),
        Operator::mul(),
        Operator::add(),
        Token::new_number(Numeric::from_integer(4)),
        Operator::add()
    ];

    assert_eq!(infix_to_postfix(&infix1), InfixConversionResult::Tokens(postfix1));
    assert_eq!(infix_to_postfix(&infix1), InfixConversionResult::InfixConversionError(InfixConversionError::MismatchedParenthesis));
}