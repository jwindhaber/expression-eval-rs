
extern crate alloc;

use alloc::vec::Vec;
use crate::tokenizer::{Parenthesis, Token};

pub fn convert_infix_to_postfix_notation(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    tokens.into_iter().for_each(|current_token| {

        match &current_token {

            Token::Operator(current_operator) => {
                while let Some(queued_operator_token) = operator_stack.pop() {
                    match &queued_operator_token {
                        Token::Operator(queued_operator) => {
                            if (queued_operator.precedence < current_operator.precedence) || current_operator.left_associative  {
                                operator_stack.push(queued_operator_token);
                                break;
                            } else {
                                output_queue.push(queued_operator_token);
                            }
                        }
                        Token::Parenthesis(Parenthesis::LeftParenthesis) => {
                            operator_stack.push(queued_operator_token);
                            break;
                        }
                        _ => {}
                    }
                }
                operator_stack.push(current_token);
            }

            Token::Literal(_) => {
                output_queue.push(current_token)
            }

            Token::Variable(_) => {
                //TODO resolve the variable and pus the literal
                output_queue.push(current_token)
            }

            Token::Parenthesis(Parenthesis::LeftParenthesis) => {
                operator_stack.push(current_token);
            }

            Token::Parenthesis(Parenthesis::RightParenthesis) => {
                while let Some(queued_operator_token) = operator_stack.pop() {
                    if queued_operator_token == Token::Parenthesis(Parenthesis::LeftParenthesis) {
                        break;
                    }
                    output_queue.push(queued_operator_token);
                }
                operator_stack.push(current_token);
            }

        }
    });

    while let Some(queued_operator_token) = operator_stack.pop() {
        output_queue.push(queued_operator_token);
    }
    Ok(output_queue)
}


// XXXXXXXXXXXXXXXXXX TESTS XXXXXXXXXXXXXXXXXX

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::vec::Vec;
    use crate::converter::convert_infix_to_postfix_notation;
    use crate::Literal::Decimal;
    use crate::tokenizer::Literal::Integer;
    use crate::tokenizer::{LESS_OR_EQUAL_OPERATOR, AND_OPERATOR, string_to_tokens, GREATER_OR_EQUAL_OPERATOR, Token, MULTIPLY_OPERATOR, MINUS_OPERATOR, POWER_OF_OPERATOR, DIVIDE_OPERATOR, PLUS_OPERATOR};
    use crate::tokenizer::Token::{Literal};


    #[test]
    fn simple_infix_to_postfix_conversion() {
        const INTEGER_LITERAL: Token = Literal(Integer(3));
        const DECIMAL_LITERAL: Token = Literal(Decimal(23.8));

        let input = Vec::from([INTEGER_LITERAL, LESS_OR_EQUAL_OPERATOR, INTEGER_LITERAL, AND_OPERATOR, DECIMAL_LITERAL, GREATER_OR_EQUAL_OPERATOR, DECIMAL_LITERAL]);
        let converted_input = convert_infix_to_postfix_notation(input).unwrap();

        let expected_output = Vec::from([INTEGER_LITERAL, INTEGER_LITERAL, LESS_OR_EQUAL_OPERATOR, DECIMAL_LITERAL, DECIMAL_LITERAL, GREATER_OR_EQUAL_OPERATOR, AND_OPERATOR]);

        assert_eq!(converted_input, expected_output);

    }

    #[test]
    fn convert_with_parenthesis() {
        let input = string_to_tokens("3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3").unwrap();
        let converted_tokens = convert_infix_to_postfix_notation(input).unwrap();

        let expected_output = Vec::from([Literal(Integer(3)), Literal(Integer(4)), Literal(Integer(2)), MULTIPLY_OPERATOR, Literal(Integer(1)), Literal(Integer(5)), MINUS_OPERATOR, Literal(Integer(2)), Literal(Integer(3)), POWER_OF_OPERATOR, POWER_OF_OPERATOR, DIVIDE_OPERATOR, PLUS_OPERATOR]);

        assert_eq!(expected_output, converted_tokens);
    }
}