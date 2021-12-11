
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
    use crate::tokenizer::Literal::Integer;
    use crate::tokenizer::{OperatorProperties, string_to_tokens};
    use crate::tokenizer::Operator::{Divide, Minus, Multiply, Plus, PowerOf};
    use crate::tokenizer::Token::{Literal, Operator};


    #[test]
    fn convert() {
        let some_input = string_to_tokens("3 < 4 && 23.8 >= 40.4 ").unwrap();
        // let some_input = Vec::from([Literal(Integer(3)), Operator(LessOrEqual), Literal(Integer(4)), Operator(And), Literal(Decimal(23.8)), Operator(GreaterOrEqual), Literal(Decimal(40.4))]);
        // println!("{:?}", some_input);

        let converted_tokens = convert_infix_to_postfix_notation(some_input).unwrap();
        // println!("{:?}", converted_tokens);
    }

    #[test]
    fn convert_with_parenthesis() {
        let expected = Vec::from([Literal(Integer(3)), Literal(Integer(4)), Literal(Integer(2)), Operator(OperatorProperties { precedence: 6, symbol: "*", left_associative: false, operator: Multiply }), Literal(Integer(1)), Literal(Integer(5)), Operator(OperatorProperties { precedence: 5, symbol: "-", left_associative: false, operator: Minus }), Literal(Integer(2)), Literal(Integer(3)), Operator(OperatorProperties { precedence: 7, symbol: "^", left_associative: true, operator: PowerOf }), Operator(OperatorProperties { precedence: 7, symbol: "^", left_associative: true, operator: PowerOf }), Operator(OperatorProperties { precedence: 6, symbol: "/", left_associative: false, operator: Divide }), Operator(OperatorProperties { precedence: 5, symbol: "+", left_associative: false, operator: Plus })]);

        // let expected = Vec::from([Literal(Integer(3)), Literal(Integer(4)), Literal(Integer(2)), Operator(OperatorProperties { precedence: 6, symbol: "*", left_associative: false  }), Literal(Integer(1)), Literal(Integer(5)), Operator(OperatorProperties { precedence: 5, symbol: "-", left_associative: false }), Literal(Integer(2)), Literal(Integer(3)), Operator(OperatorProperties { precedence: 7, symbol: "^", left_associative: true }), Operator(OperatorProperties { precedence: 7, symbol: "^", left_associative: true }), Operator(OperatorProperties { precedence: 6, symbol: "/", left_associative: false }), Operator(OperatorProperties { precedence: 5, symbol: "+", left_associative: false })]);

        let given = string_to_tokens("3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3").unwrap();
        // let some_input = Vec::from([Literal(Integer(3)), Operator(LessOrEqual), Literal(Integer(4)), Operator(And), Literal(Decimal(23.8)), Operator(GreaterOrEqual), Literal(Decimal(40.4))]);

        // println!("{:?}", given);
        let converted_tokens = convert_infix_to_postfix_notation(given).unwrap();

        assert_eq!(expected, converted_tokens);

        // println!("{:?}", converted_tokens);
    }
}