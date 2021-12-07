use crate::tokenizer::{Parenthesis, Token};

fn convert_infix_to_postfix_notation(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    tokens.into_iter().for_each(|current_token| {

        match &current_token {

            Token::Operator(current_operator) => {
                while let Some(queued_operator_token) = operator_stack.pop() {
                    match &queued_operator_token {
                        Token::Operator(queued_operator) => {
                            if *queued_operator as u8 <= *current_operator as u8 {
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
    use crate::converter::convert_infix_to_postfix_notation;
    use crate::tokenizer::string_to_tokens;


    #[test]
    fn convert() {
        let some_input = string_to_tokens("3 < 4 && 23.8 >= 40.4 ").unwrap();
        // let some_input = Vec::from([Literal(Integer(3)), Operator(LessOrEqual), Literal(Integer(4)), Operator(And), Literal(Decimal(23.8)), Operator(GreaterOrEqual), Literal(Decimal(40.4))]);
        println!("{:?}", some_input);

        let converted_tokens = convert_infix_to_postfix_notation(some_input).unwrap();
        println!("{:?}", converted_tokens);
    }

    #[test]
    fn convert_with_parenthesis() {
        let some_input = string_to_tokens("3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3").unwrap();
        // let some_input = Vec::from([Literal(Integer(3)), Operator(LessOrEqual), Literal(Integer(4)), Operator(And), Literal(Decimal(23.8)), Operator(GreaterOrEqual), Literal(Decimal(40.4))]);
        println!("{:?}", some_input);

        let converted_tokens = convert_infix_to_postfix_notation(some_input).unwrap();
        println!("{:?}", converted_tokens);
    }
}