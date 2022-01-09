#![cfg_attr(not(feature = "std"), no_std)]
// #![cfg_attr(not(test), no_std)]
#[cfg(test)] #[macro_use]
extern crate assert_matches;

mod tokenizer;
mod eval;
mod converter;





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
