//!
//! Tokenizer which takes a string as an input and generates a list of tokens out of it.
//!
//! Since we are dealing with mathematical expressions we have to following flavours of tokens:
//!
//! literal -> 3 44 4.5
//! variable -> a someName
//! operator -> + - / * && ||

// use std::iter::Peekable;
// use std::str::Chars;

extern crate alloc;


use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::Peekable;
use core::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(OperatorProperties),
    Literal(Literal),
    Variable(Box<str>),
    Parenthesis(Parenthesis),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(Box<str>),
    Boolean(bool),
    Decimal(f64),
    Integer(i64),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Or,
    And,
    Not,

    NotEqual,
    Equal,

    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    Plus,
    Minus,
    Divide,
    Multiply,

    PowerOf,

}

// impl Token {
//     pub fn getSome(&self) -> OperatorProperties {
//         if let &Token::Operator(i) = self {
//             i
//         }
//         else {
//             panic!("called MyEnum::FooBarBaz() on {:?}", self)
//         }
//     }
//
// }

const OR_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "||", precedence: 1, left_associative: false, operator: Operator::Or});

const AND_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "&&", precedence: 2, left_associative: false, operator: Operator::And });

const NOT_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "!=", precedence: 3, left_associative: false, operator: Operator::NotEqual });
const EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "==", precedence: 3, left_associative: false, operator: Operator::Equal });

const GREATER_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: ">", precedence: 4, left_associative: false, operator: Operator::Greater });
const GREATER_OR_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: ">=", precedence: 4, left_associative: false, operator: Operator::GreaterOrEqual });
const LESS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "<", precedence: 4, left_associative: false, operator: Operator::Less });
const LESS_OR_EQUAL_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "<=", precedence: 4, left_associative: false, operator: Operator::LessOrEqual });


const PLUS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "+", precedence: 5, left_associative: false, operator: Operator::Plus });
const MINUS_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "-", precedence: 5, left_associative: false, operator: Operator::Minus });
const DIVIDE_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "/", precedence: 6, left_associative: false, operator: Operator::Divide });
const MULTIPLY_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "*", precedence: 6, left_associative: false, operator: Operator::Multiply });

const POWER_OF_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "^", precedence: 7, left_associative: true, operator: Operator::PowerOf });

const NOT_OPERATOR: Token = Token::Operator(OperatorProperties { symbol: "!", precedence: 8, left_associative: false, operator: Operator::Not });


#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OperatorProperties {
    pub precedence: i8,
    pub symbol: &'static str,
    pub left_associative: bool,
    pub operator: Operator,
}




pub fn string_to_tokens(expression_string: &str) -> Result<Vec<Token>, &'static str>
{
    let mut result: Vec<Option<Token>> = Vec::new();
    let mut iter = expression_string.chars().peekable();


    while let Some(character) = iter.next() {
        let operator: Option<Token> = match character {
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
        token = Token::Literal(Literal::Boolean(true))
    }
    if token_string == "false" {
        token = Token::Literal(Literal::Boolean(false))
    }

    return Some(token);
}

fn extract_operator_simple(expression_string_iterator: &mut Peekable<Chars>, operator: Token, second_operator: Token, expected: char) -> Option<Token> {
    return match expression_string_iterator.peek() {
        Some(value) => {
            if *value == expected {
                Some(operator);
            }
            Some(second_operator)
        }
        None => { None }
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
    use crate::tokenizer::{Literal, string_to_tokens, Token};


    #[test]
    fn simple_boolean_expression() {
        let x = "false";
        let vec = string_to_tokens(x).unwrap();
        let result = vec.first().unwrap();
        assert_eq!(*result, Token::Literal(Literal::Boolean(false)));
    }

    #[test]
    fn some_test() {
        let x = "3 < 4 && 23.8 >= 40.4 ";
        let result = string_to_tokens(x);

        // println!("{:?}", result);
    }
}