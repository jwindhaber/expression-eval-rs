// Support using expression eval without the standard library!
#![cfg_attr(not(feature = "std"), no_std)]

// for the tests lets bring assert_matches and the std crate into scope
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[cfg(test)]
#[macro_use]
extern crate std;
// #[cfg(test)] use std::prelude::*;
extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use crate::context::replace_variables_with_values_from_context;
use crate::converter::convert_infix_to_postfix_notation;
use crate::eval::evaluate_tokens;
use crate::tokenizer::{Literal, string_to_tokens};

mod tokenizer;
mod eval;
mod converter;
mod context;


pub fn evaluate_expression(expression_string: &str) -> Result<Literal, &'static str> {
    string_to_tokens(expression_string)
        .and_then(convert_infix_to_postfix_notation)
        .and_then(evaluate_tokens)
}


pub fn evaluate_expression_with_context(expression_string: &str, context: BTreeMap<&str, &str>) -> Result<Literal, &'static str> {
    let result = string_to_tokens(expression_string);

    match result {
        Ok(tokens) => {
            replace_variables_with_values_from_context(tokens, context)
                .and_then(convert_infix_to_postfix_notation)
                .and_then(evaluate_tokens)
        }
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::collections::btree_map::BTreeMap;
    use crate::evaluate_expression_with_context;
    use crate::Literal::Decimal;

    #[test]
    fn simple_infix_to_postfix_conversion() {
        let mut context = BTreeMap::new();
        context.insert("first", "4.5");
        context.insert("second", "3");

        let result = evaluate_expression_with_context("first + second", context);

        assert_eq!(Ok(Decimal(7.5)), result);
    }



}