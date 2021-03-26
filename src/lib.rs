mod util;
mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::util::{OP, Literal};

    /*
    Find tests in their respective files
     */

    #[test]
    fn lex_and_parse_two_plus_two () {
        assert_eq!(
            parse(lex("2 + 2")),
            vec![
                OP::Addition {
                    a: Literal::Number(2),
                    b: Literal::Number(2),
                },
            ]
        )
    }
}
