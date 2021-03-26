/*
Util includes utilities and declarations needed in other files.
 */

#[macro_export]
macro_rules! previous_peekable {
    ( $a:expr ) => {
        PrevPeekable::new($a);
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct Number(pub i32);

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
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

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(i32),      // type: i32
    String(String),   // type: String
}

#[derive(PartialEq, Debug, Clone)]
pub struct Addition {
    pub left: Literal,
    pub right: Literal,
}

#[derive(PartialEq, Debug, Clone)]
pub enum OP {
    Addition {
        a: Literal,
        b: Literal,
    },
}