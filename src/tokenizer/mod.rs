//!
//! Tokenizer which takes a string as an input and generates a list of tokens out of it.
//!
//! Since we are dealing with mathematical expressions we have to following flavours of tokens:
//!
//! literal -> 3 44 4.5
//! variable -> a someName
//! operator -> + - / * && ||

use std::iter::Peekable;
use std::str::Chars;




#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Literal(Literal),
    Variable(Box<str>),
    Parenthesis(Parenthesis)
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(Box<str>),
    Boolean(bool),
    Decimal(f64),
    Integer(i64)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Or = 1,
    And = 2,
    Not = 4,

    NotEqual = 5,
    Equal = 6,

    Greater = 7,
    GreaterOrEqual = 8,
    Less = 9,
    LessOrEqual = 10,

    Plus = 11,
    Minus = 12,
    Divide = 13,
    Multiply = 14,

    PowerOf = 15,


}

#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    LeftParenthesis,
    RightParenthesis
}

pub fn string_to_tokens(expression_string: &str) -> Result<Vec<Token>, &'static str>
{
    let mut result:Vec<Option<Token>> = Vec::new();
    let mut iter = expression_string.chars().peekable();


    while let Some(character) = iter.next() {

        let operator:Option<Token> = match character {
            'A'..='Z' | 'a'..='z' => {
                extract_variable(&mut iter, character)
            }
            '0'..='9' => {
                extract_number(&mut iter, character)
            }
            '|' => {
               extract_operator(&mut iter, Operator::Or, '|')
            }
            '&' => {
                extract_operator(&mut iter, Operator::And, '&')
            }
            '=' => {
                extract_operator(&mut iter, Operator::Equal, '=')
            }
            '!' => {
                extract_operator_simple(&mut iter, Operator::Not, Operator::NotEqual,'=')
            }
            '<' => {
                extract_operator_simple(&mut iter, Operator::Less, Operator::LessOrEqual,'=')
            }
            '>' => {
                extract_operator_simple(&mut iter, Operator::Greater, Operator::GreaterOrEqual,'=')
            }
            '^' => {
                Some(Token::Operator(Operator::PowerOf))
            }
            '+' => {
                Some(Token::Operator(Operator::Plus))
            }
            '-' | '−' => {
                Some(Token::Operator(Operator::Minus))
            }
            '*' | '×' => {
                Some(Token::Operator(Operator::Multiply))
            }
            '/' | '÷' => {
                Some(Token::Operator(Operator::Divide))
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
                break
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
                break
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

fn extract_operator_simple(expression_string_iterator: &mut Peekable<Chars>, operator: Operator, second_operator: Operator, expected: char) -> Option<Token> {
    return match expression_string_iterator.peek() {
        Some(value) => {
            if *value == expected {
                Some(Token::Operator(operator));
            }
            Some(Token::Operator(second_operator))
        }
        None => { None }
    }

}


fn extract_operator(expression_string_iterator: &mut Peekable<Chars>, operator: Operator, expected: char) -> Option<Token> {
    match expression_string_iterator.peek() {
        Some(value) => {
            if *value == expected {
                return Some(Token::Operator(operator));
            }
            return None
        }
        None => { return None}
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

        println!("{:?}", result);
    }


}