mod evaluation;

extern crate alloc;

use crate::tokenizer::{Operator, Token, Literal};

use alloc::vec::Vec;
use core::ops::{Div, Mul, Sub, Add};
use crate::eval::evaluation::{eval_boolean_literals, eval_decimal_literals, eval_integer_literals};

pub fn evaluate_expression(tokens: Vec<Token>) -> Result<Literal, &'static str> {
    let mut stack: Vec<Literal> = Vec::new();

    tokens.into_iter().for_each(|token| {
        match token {
            Token::Operator(operator_properties) => {
                let right_side = stack.pop().unwrap();
                let left_side = stack.pop().unwrap();
                let operator = operator_properties.operator;

                let result = evaluate_operator(&right_side, &left_side, operator);
                //TODO error handling
                if let Ok(literal) = result { stack.push(literal)}


            }
            Token::Literal(literal) => {
                stack.push(literal);
            }

            _ => {
                //TODO panic if occurs
            }
        }
    });


    stack.pop().ok_or("something is wrong")
}

fn evaluate_operator(right_side: &Literal, left_side: &Literal, operator: Operator) -> Result<Literal, &'static str> {
    if let (Literal::Integer(left), Literal::Integer(right)) = (&left_side, &right_side) {
        eval_integer_literals(operator, *left, *right)

    } else if let (Literal::Decimal(left), Literal::Decimal(right)) = (&left_side, &right_side) {
        // both are decimals
        eval_decimal_literals(operator, *left, *right)

    } else if let (Literal::Decimal(left), Literal::Integer(right_integer)) = (&left_side, &right_side) {
        // different transform to decimal
        let right = *right_integer as f64;
        eval_decimal_literals(operator, *left, right)

    } else if let (Literal::Integer(left_integer), Literal::Decimal(right)) = (&left_side, &right_side) {
        // different transform to decimal
        let left = *left_integer as f64;
        eval_decimal_literals(operator, left, *right)

    } else if let (Literal::Boolean(left), Literal::Boolean(right)) = (&left_side, &right_side) {
        eval_boolean_literals(operator, *left, *right)

    } else {
        Err("Either the left or the right literal is not supported")

    }
}


// XXXXXXXXXXXXXXXXXX TESTS XXXXXXXXXXXXXXXXXX

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::converter::convert_infix_to_postfix_notation;
    use crate::eval::{evaluate_expression};
    use crate::tokenizer::{Literal, string_to_tokens};


    #[test]
    fn convert() {
        let given = string_to_tokens("( 1.0 + 5 ) / 2 + 3.0 > 5 && 6 < 5").unwrap();
        let converted_tokens = convert_infix_to_postfix_notation(given).unwrap();
        let expression = evaluate_expression(converted_tokens).unwrap();

        // assert_eq!(expected, converted_tokens);
        // match expression {
        //     Literal::String(_) => {}
        //     Literal::Boolean(_) => {}
        //     Literal::Decimal(_) => {}
        //     Literal::Integer(value) => {
        //         println!("{:?}", value);
        //     }
        // }


        // println!("{:?}", expression);
    }
}