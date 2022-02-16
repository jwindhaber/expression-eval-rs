//!
//! Tokenizer which takes a string as an input and generates a list of tokens out of it.
//!
//! Since we are dealing with mathematical expressions we have to following flavours of tokens:
//!
//! literal -> 3 44 4.5
//! variable -> a someName
//! operator -> + - / * && ||
//!
//! regex
//! (?:[<>=!]=?|=|&&|\|\||\^|-?[0-9][0-9,\.]*|[A-z]*)




extern crate alloc;


use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::Peekable;
use core::str::Chars;
use crate::definition::{Operator, OperatorProperties, Parenthesis, Token};
use crate::Literal;
use crate::Literal::Boolean;






pub const OR_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "||", precedence: 1, left_associative: false, operator: Operator::Or});

pub const AND_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "&&", precedence: 2, left_associative: false, operator: Operator::And });

pub const NOT_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "!=", precedence: 3, left_associative: false, operator: Operator::NotEqual });
pub const EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "==", precedence: 3, left_associative: false, operator: Operator::Equal });

pub const GREATER_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: ">", precedence: 4, left_associative: false, operator: Operator::Greater });
pub const GREATER_OR_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: ">=", precedence: 4, left_associative: false, operator: Operator::GreaterOrEqual });
pub const LESS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "<", precedence: 4, left_associative: false, operator: Operator::Less });
pub const LESS_OR_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "<=", precedence: 4, left_associative: false, operator: Operator::LessOrEqual });


pub const PLUS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "+", precedence: 5, left_associative: false, operator: Operator::Plus });
pub const MINUS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "-", precedence: 5, left_associative: false, operator: Operator::Minus });
pub const DIVIDE_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "/", precedence: 6, left_associative: false, operator: Operator::Divide });
pub const MULTIPLY_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "*", precedence: 6, left_associative: false, operator: Operator::Multiply });

pub const POWER_OF_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "^", precedence: 7, left_associative: true, operator: Operator::PowerOf });

pub const NOT_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "!", precedence: 8, left_associative: false, operator: Operator::Not });

pub const TRUE: Token = Token::Literal(Boolean(true));
pub const FALSE: Token = Token::Literal(Boolean(false));







pub fn string_to_tokens(expression_string: &str) -> Result<Vec<Token>, &'static str>
{
    let mut result: Vec<Option<Token>> = Vec::new();
    let mut iter = expression_string.chars().peekable();


    while let Some(character) = iter.next() {
        let operator: Option<Token> = match character {
            '\'' | '\"' => {
                extract_string_literal(&mut iter, character)
            }
            'A'..='Z' | 'a'..='z' => {
                extract_variable(&mut iter, character)
            }
            '0'..='9' => {
                extract_number(&mut iter, character)
            }
            '|' => {
                extract_operator(&mut iter, OR_OPERATOR, '|')
            }
            '&' => {
                extract_operator(&mut iter, AND_OPERATOR, '&')
            }
            '=' => {
                extract_operator(&mut iter, EQUAL_OPERATOR, '=')
            }
            '!' => {
                extract_operator_simple(&mut iter, NOT_OPERATOR, NOT_EQUAL_OPERATOR, '=')
            }
            '<' => {
                extract_operator_simple(&mut iter, LESS_OPERATOR, LESS_OR_EQUAL_OPERATOR, '=')
            }
            '>' => {
                extract_operator_simple(&mut iter, GREATER_OPERATOR, GREATER_OR_EQUAL_OPERATOR, '=')
            }
            '^' => {
                Some(POWER_OF_OPERATOR)
            }
            '+' => {
                Some(PLUS_OPERATOR)
            }
            '-' | '−' => {
                Some(MINUS_OPERATOR)
            }
            '*' | '×' => {
                Some(MULTIPLY_OPERATOR)
            }
            '/' | '÷' => {
                Some(DIVIDE_OPERATOR)
            }
            ')' => {
                Some(Token::Parenthesis(Parenthesis::RightParenthesis))
            }
            '(' => {
                Some(Token::Parenthesis(Parenthesis::LeftParenthesis))
            }
            _ => {
                None
            }
        };
        result.push(operator);
    }
    let filtered_result = result.into_iter().filter_map(|e| e).collect();
    return Ok(filtered_result);
}

fn extract_number(expression_string_iterator: &mut Peekable<Chars>, character: char) -> Option<Token> {
    let mut is_integer = true;
    let mut number_string = String::new();
    number_string.push(character);

    while let Some(character) = expression_string_iterator.next() {
        match character {
            '0'..='9' => {
                number_string.push(character);
            }
            '.' => {
                is_integer = false;
                number_string.push(character);
            }
            _ => {
                break;
            }
        }
    }

    let literal = if is_integer {
        let result = number_string.parse::<i64>().unwrap();
        Literal::Integer(result)
    } else {
        let result = number_string.parse::<f64>().unwrap();
        Literal::Decimal(result)
    };

    return Some(Token::Literal(literal));
}

fn extract_string_literal(expression_string_iterator: &mut Peekable<Chars>, character: char) -> Option<Token> {
    let mut token_string = String::new();

    while let Some(character) = expression_string_iterator.next() {
        match character {
            'A'..='Z' | 'a'..='z' => {
                token_string.push(character);
            }
            _ => {
                break;
            }
        }
    }

    let mut token = Token::Literal(Literal::String(Box::from(token_string.clone())));

    return Some(token);
}

fn extract_variable(expression_string_iterator: &mut Peekable<Chars>, character: char) -> Option<Token> {
    let mut token_string = String::new();
    token_string.push(character);

    while let Some(character) = expression_string_iterator.next() {
        match character {
            'A'..='Z' | 'a'..='z' => {
                token_string.push(character);
            }
            _ => {
                break;
            }
        }
    }

    let mut token = Token::Variable(Box::from(token_string.clone()));


    //TODO move this out of here
    if token_string == "true" {
        token = TRUE
    }
    if token_string == "false" {
        token = FALSE
    }

    return Some(token);
}

fn extract_operator_simple(expression_string_iterator: &mut Peekable<Chars>, operator: Token, second_operator: Token, expected: char) -> Option<Token> {
    let next_char = expression_string_iterator.peek();
    return match next_char {
        Some(value) => {
            if *value == expected {
                return Some(second_operator);
            }
            return Some(operator);
        }
        None => { Some(operator) }
    };

}


fn extract_operator(expression_string_iterator: &mut Peekable<Chars>, operator: Token, expected: char) -> Option<Token> {
    match expression_string_iterator.peek() {
        Some(value) => {
            if *value == expected {
                return Some(operator);
            }
            return None;
        }
        None => { return None; }
    }
}


// XXXXXXXXXXXXXXXXXX TESTS XXXXXXXXXXXXXXXXXX

#[cfg(test)]
mod tests {
    use std::prelude::v1::{Box, Vec};
    use crate::tokenizer::{AND_OPERATOR, Literal, Operator, string_to_tokens, Token};
    use rstest::rstest;
    use crate::Literal::Boolean;



    #[test]
    fn simple_string_literal_expression() {
        let a = "\"A\"";
        let vec = string_to_tokens(a).unwrap();
        let result = vec.first().unwrap();
        assert_eq!(*result, Token::Literal(Literal::String(Box::from("A"))));

        let b = "\'B\'";
        let vec = string_to_tokens(b).unwrap();
        let result = vec.first().unwrap();
        assert_eq!(*result, Token::Literal(Literal::String(Box::from("B"))));

        let c = "somevar == 'C'";
        let vec = string_to_tokens(c).unwrap();
        let result = vec.get(2).unwrap();
        assert_eq!(*result, Token::Literal(Literal::String(Box::from("C"))));

        let d = "somevar == 'D'";
        let vec = string_to_tokens(d).unwrap();
        let result = vec.get(2).unwrap();
        assert_eq!(*result, Token::Literal(Literal::String(Box::from("D"))));


    }

    #[test]
    fn simple_boolean_literal_expression() {
        let x = "false";
        let vec = string_to_tokens(x).unwrap();
        let result = vec.first().unwrap();
        assert_eq!(*result, Token::Literal(Literal::Boolean(false)));
    }

    #[test]
    fn simple_integer_literal_expression() {
        let x = "3";
        let vec = string_to_tokens(x).unwrap();
        let result = vec.first().unwrap();
        assert_eq!(*result, Token::Literal(Literal::Integer(3)));
    }

    #[test]
    fn simple_less_operator_expression() {
        let x = "<=";
        let vec = string_to_tokens(x).unwrap();
        let result = vec.get(0).unwrap();

        assert_matches!(result, Token::Operator(properties) => {
            assert_eq!(properties.operator, Operator::LessOrEqual);
        });
    }

    #[test]
    fn simple_boolean() {

        let x = "true && false";
        let result = string_to_tokens(x).unwrap();
        let expected = Vec::from([Token::Literal(Boolean(true)), AND_OPERATOR, Token::Literal(Boolean(false))]);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::or("||", Operator::Or)]
    #[case::and("&&", Operator::And)]
    #[case::not("!", Operator::Not)]
    #[case::not_equal("!=", Operator::NotEqual)]
    #[case::equal("==", Operator::Equal)]
    #[case::greater(">", Operator::Greater)]
    #[case::greater_or_equal(">=", Operator::GreaterOrEqual)]
    #[case::less("<", Operator::Less)]
    #[case::less_or_equal("<=", Operator::LessOrEqual)]
    #[case::plus("+", Operator::Plus)]
    #[case::minus("-", Operator::Minus)]
    #[case::divide("/", Operator::Divide)]
    #[case::multiply("*", Operator::Multiply)]
    #[case::power_of("^", Operator::PowerOf)]
    fn simple_parametrized_operator_expression(#[case] expression: &str,#[case] expected: Operator) {
        let vec = string_to_tokens(expression).unwrap();
        let result = vec.get(0).unwrap();

        assert_matches!(result, Token::Operator(properties) => {
            assert_eq!(properties.operator, expected);
        });
    }

}