// Support using expression eval without the standard library!
#![cfg_attr(not(feature = "std"), no_std)]

// for the tests lets bring assert_matches and the std crate into scope
#[cfg(test)] #[macro_use] extern crate assert_matches;
#[cfg(test)] #[macro_use] extern crate std;
#[cfg(test)] use std::prelude::*;

use crate::converter::convert_infix_to_postfix_notation;
use crate::eval::evaluate_tokens;
use crate::tokenizer::{Literal, string_to_tokens};

mod tokenizer;
mod eval;
mod converter;


pub fn evaluate_expression(expression_string: &str) -> Result<Literal, &'static str> {
    string_to_tokens(expression_string)
        .and_then(convert_infix_to_postfix_notation)
        .and_then(evaluate_tokens)
}