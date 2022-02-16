extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use crate::definition::Token;
use crate::{Literal, string_to_tokens};



pub fn replace_variables_with_values_from_context(tokens: Vec<Token>, context: &BTreeMap<&str, &str>) -> Result<Vec<Token>, &'static str> {
    let x: Vec<Token> = tokens.into_iter()
        .flat_map(|element| find_and_replace(element, context))
        .collect();

    Ok(x)
}

fn find_and_replace(token: Token, context: &BTreeMap<&str, &str>) -> Vec<Token> {
    match token {
        Token::Variable(variable) => {
            let option = context.get(&*variable).unwrap();
            let result = string_to_tokens(&option).unwrap();
            result.into_iter().map(|token| map_token_to(token)).collect()
        }
        _ => {
            Vec::from([token])
        }
    }
}

fn map_token_to(token: Token) -> Token {
    match token {
        Token::Variable(variable) => {
            Token::Literal(Literal::String(variable))
        }
        _ => { token }
    }

}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::collections::btree_map::BTreeMap;
    use std::prelude::rust_2021::Vec;
    use crate::context::replace_variables_with_values_from_context;
    use crate::definition::Token::Literal;
    use crate::string_to_tokens;
    use crate::Literal::{Boolean, Decimal, Integer};
    use crate::tokenizer::{EQUAL_OPERATOR, GREATER_OPERATOR};


    #[test]
    fn simple_replacements_of_variables() {
        let mut some_map = BTreeMap::new();
        some_map.insert("first", "4.5");
        some_map.insert("second", "3");
        some_map.insert("third", "true");

        let tokens = string_to_tokens("first > second == third").unwrap();
        let vec = replace_variables_with_values_from_context(tokens, &some_map).unwrap();

        let expected = Vec::from([Literal(Decimal(4.5)), GREATER_OPERATOR, Literal(Integer(3)), EQUAL_OPERATOR, Literal(Boolean(true))]);

        assert_eq!(vec, expected);
    }

    #[test]
    fn complex_replacements_of_variables() {
        let mut some_map = BTreeMap::new();
        some_map.insert("first", "4.5 > 3");
        some_map.insert("second", "true");

        let tokens = string_to_tokens("first == second").unwrap();
        let vec = replace_variables_with_values_from_context(tokens, &some_map).unwrap();

        let expected = Vec::from([Literal(Decimal(4.5)), GREATER_OPERATOR, Literal(Integer(3)), EQUAL_OPERATOR, Literal(Boolean(true))]);

        assert_eq!(vec, expected);
    }
}