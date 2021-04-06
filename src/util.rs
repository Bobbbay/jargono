/*
Util includes utilities and declarations needed in other files.
 */

#[macro_export]
macro_rules! previous_peekable {
    ( $a:expr ) => {
        PrevPeekable::new($a);
    };
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number     (i64),
    Str        (String),
    Boolean    (bool),

    Reference  (String),
    Assign     (String,    Box<Expr>),
    Add        (Box<Expr>, Box<Expr>),
    Subtract   (Box<Expr>, Box<Expr>),
    Multiply   (Box<Expr>, Box<Expr>),
    Divide     (Box<Expr>, Box<Expr>),
    If         (Box<Expr>, Vec<Expr>, Vec<Expr>),
}