use crate::util::{BinaryOp, Node, Type, UnaryOp};
use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_until, take_while};
use nom::character::complete::one_of;
use nom::character::complete::{alphanumeric1 as alphanumeric, char};
use nom::character::is_digit;
use nom::combinator::{consumed, cut, map, map_parser, map_res, value};
use nom::error::ErrorKind;
use nom::error::{context, ContextError, ParseError};
use nom::number::complete::le_i32;
use nom::sequence::{preceded, terminated};
use nom::IResult;

use crate::util::Node::{BinaryExpr, Bool, Function, UnaryExpr};
use crate::util::UnaryOp::Return;

pub fn parse(input: String) -> Vec<Node> {
    let res =
        parse_jargono::<(&str, ErrorKind)>(&*input).expect("Error unwrapping parsed information");

    /* An example AST
    vec![
        Function {
            name: "main".to_string(),
            arguments: vec![],
            return_value: Type::Bool,
            children: vec![Box::from(UnaryExpr {
                op: Return,
                child: Box::from(Bool(true)),
            })],
        },
        Function {
            name: "add".to_string(),
            arguments: vec![],
            return_value: Type::Int,
            children: vec![Box::from(UnaryExpr {
                op: UnaryOp::Return,
                child: Box::from(BinaryExpr {
                    op: BinaryOp::Plus,
                    lhs: Box::new(Node::Int(1)),
                    rhs: Box::new(Node::Int(2)),
                }),
            })],
        },
    ]
     */

    vec![res.1]
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric, '\\', one_of("\"n\\"))(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn return_identifier<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    tag("return")(input)
}

fn parse_jargono<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Node, E> {
    preceded(
        sp,
        alt((
            // map(string, |s| JsonValue::Str(String::from(s))),
            // map(le_i32, Node::Int),
            map(boolean, Node::Bool),
            map(boolean, Node::Bool),
            map(return_identifier, |s: &str| {
                let a = return_identifier::<(&str, ErrorKind)>(i).unwrap().0;

                Node::UnaryExpr {
                    op: UnaryOp::Return,
                    child: Box::new(parse_jargono::<(&str, ErrorKind)>(a).unwrap().1),
                }
            }),
        )),
    )(i)
}

/*
extern crate nom;

use crate::util::{ Node, BinaryOp };

pub(self) fn declaration() -> Result<Node, &str> {
    Ok(Node::Variable(
        "nameOfVar".parse().unwrap(),
        Box::from(number("25").unwrap()),
    ))
}

/// `number` will take an input string and try to turn it into a number.
///
/// # Arguments
///
/// * `input` a string to be parsed.
///
/// # See also
///
/// [`literal`], [`string`], [`boolean`]
pub(self) fn number(input: &str) -> Result<Node, &str> {
    let res = try_number(input);

    Ok(Node::Number(res.unwrap().1))
}

/// `string` has the goal of consuming the preceding and proceeding `"`s, while simultaneously
/// returning the value inside.
///
/// # Arguments:
///
/// * `input` a string to be parsed.
///
/// # See also
///
/// [`literal`], [`number`], [`boolean`]
pub(self) fn string(input: &str) -> Result<Node, &str> {
    let res: IResult<&str, &str, nom::error::Error<&str>> = delimited (
        tag("\""),
        is_not("\""),
        tag("\"")
    )(input);

    Ok(Node::Str(res.unwrap().1.parse().unwrap()))
}

/// `boolean` will take a string as input and figure out whether it contains `true` or false`.
///
/// # Arguments
///
/// * `input` a string to be parsed.
///z
/// # See also
///
/// [`literal`], [`string`], [`number`]
pub(self) fn boolean(input: &str) -> Result<Node, &str> {
    match input {
        "true"  => Ok(Node::Boolean(true)),
        "false" => Ok(Node::Boolean(false)),
        _       => Err(Error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(
            number("777"),
            Ok(Node::Number(777))
        )
    }

    #[test]
    fn strings() {
        assert_eq!(
            string("\"Hello, world.\""),
            Ok(Node::Str("Hello, world.".parse().unwrap()))
        )
    }

    #[test]
    fn booleans() {
        assert_eq!(
            boolean("true"),
            Ok(Node::Boolean(true))
        );

        assert_eq!(
            boolean("false"),
            Ok(Node::Boolean(false))
        )
    }

    #[test]
    fn declarations() {
        assert_eq!(
            declaration(),
            Ok(Node::Variable("nameOfVar".parse().unwrap(), Box::from(Node::Number(25))))
        )
    }
}*/
