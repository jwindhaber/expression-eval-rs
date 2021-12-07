use crate::tokenizer::{Operator, Token};
use crate::tokenizer::Token::Operator;

fn convert_infix_to_postfix_notation(tokens: Vec<Token>) {//-> Result<Vec<Token>, &'static str> {

    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();


    tokens.into_iter().for_each(|token| {
        match token {
            Token::Operator(current_operator) => {
                while let Some(queued_operator_token) = operator_stack.pop() {
                    if queued_operator_token == Operator::LeftParenthesis {
                        operator_stack.push(queued_operator_token);
                        break
                    }
                    output_queue.



                }
            }
            Token::Literal(_) => {
                output_queue.push(token)
            }
            Token::Variable(_) => {}
            Token::Brace(_) => {}
        }
    });


}