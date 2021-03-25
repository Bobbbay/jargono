/*
Util includes utilities and declarations needed in other files.
 */

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Add,              // +
    Subtract,         // -
    Multiply,         // *
    Divide,           // /
    Equals,           // =

    Number(i32),      // type: i32

    Declaration,      // let
    Delimiter,        // ;

    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftCurlyBrace,   // {
    RightCurlyBrace,  // }

    Identifier,       // everything else
}