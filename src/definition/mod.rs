
use alloc::boxed::Box;


#[derive(Debug, PartialEq)]
pub enum Literal {
    String(Box<str>),
    Boolean(bool),
    Decimal(f64),
    Integer(i64),
}


#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(OperatorProperties),
    Literal(Literal),
    Variable(Box<str>),
    Parenthesis(Parenthesis),
}

#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OperatorProperties {
    pub precedence: i8,
    pub symbol: &'static str,
    pub left_associative: bool,
    pub operator: Operator,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Or,
    And,
    Not,

    NotEqual,
    Equal,

    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,

    Plus,
    Minus,
    Divide,
    Multiply,

    PowerOf,

}