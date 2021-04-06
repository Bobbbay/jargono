extern crate nom;
use nom::{
    bytes::{
        complete::{tag, take_until},
        streaming::is_not,
    },
    combinator::map_res,
    sequence::{delimited, preceded},
    character::complete::digit1,
    IResult,
};

/// `strip_comment` will quite literally strip single-lined comments from the input, and return
/// the non-commented body as well as the comment body. In Jargono, these comments are defined as
/// `//`.
///
/// # Example
///
/// ```ignore
/// strip_comments("// commentification\nlet a = 0;");
/// ```
///
/// See also: [`strip_multiline_comment`]
pub(self) fn strip_comment(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    preceded(tag("//"), take_until("\n"))(input)
}

/// `strip_multiline_comment` will quite literally strip multi-lined comments from the input, and
/// return the non-commented body as well as the comment body. In Jargono, multi-line comments are
/// specified as `/* */`.
///
/// # Example
///
/// ```ignore
/// strip_comments("/* commentification */");
/// ```
///
/// See also: [`strip_comment`]
pub(self) fn strip_multiline_comment(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    delimited(tag("/*"), is_not("*/"), tag("*/"))(input)
}

/// `number` will take an input string and try to turn it into a number.
///
/// See also: [`literal`], [`string`], [`boolean`]
pub(self) fn number(input: &str) -> IResult<&str, i64, nom::error::Error<&str>> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

/// `string` has the goal of consuming the preceding and proceeding `"`s, while simultaneously
/// returning the value inside.
///
/// See also: [`literal`], [`number`], [`boolean`]
pub(self) fn string(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    delimited(tag("\""), is_not("\""), tag("\""))(input)
}

/// `boolean` will take a string as input and figure out whether it contains `true` or false`.
///
/// See also: [`literal`], [`string`], [`number`]
pub(self) fn boolean(input: &str) -> Result<bool, &str> {
    match input {
        "true"  => Ok(true),
        "false" => Ok(false),
        _       => Err("Unknown boolean."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment() {
        assert_eq!(
            strip_comment("// commentification \n let a = 0;"),
            Ok(("\n let a = 0;", " commentification "))
        );
    }

    #[test]
    fn multiline_comment() {
        assert_eq!(
            strip_multiline_comment("/* commentification */ let a = 0;"),
            Ok((" let a = 0;", " commentification "))
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(
            number("777"),
            Ok(("", 777))
        )
    }

    #[test]
    fn strings() {
        assert_eq!(
            string("\"Hello, world.\""),
            Ok(("", "Hello, world."))
        )
    }

    #[test]
    fn booleans() {
        assert_eq!(
            boolean("true"),
            Ok(true)
        );

        assert_eq!(
            boolean("false"),
            Ok(false)
        )
    }
}