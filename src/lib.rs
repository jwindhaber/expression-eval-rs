// Support using expression eval without the standard library!
#![cfg_attr(not(feature = "std"), no_std)]

// for the tests lets bring assert_matches and the std crate into scope
#[cfg(test)] #[macro_use] extern crate assert_matches;
#[cfg(test)] #[macro_use] extern crate std;
#[cfg(test)] use std::prelude::*;

mod tokenizer;
mod eval;
mod converter;



