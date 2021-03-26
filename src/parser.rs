use crate::util::{ Token, Literal, OP};
use prev_iter::PrevPeekable;
use crate::previous_peekable;

pub(crate) fn parse (toks: Vec<Token>) -> Vec<OP> {
    let mut opcode = vec![];

    let mut tokens = previous_peekable!(toks.iter().enumerate().peekable());

    while let Some((_i, token)) = tokens.next() {
        match token {
            Token::Number(_val) => (),
            Token::Plus => {
                if let Token::Number(a) = tokens.prev_peek().unwrap().1 {
                    if let Token::Number(b) =  tokens.peek().unwrap().1 {

                        opcode.push(OP::Addition {
                            a: Literal::Number(*a),
                            b: Literal::Number(*b),
                        });

                        continue;
                    }
                    continue;
                };
                panic!("You can't add non-integer types!");
            },
            _ => panic!("Unknown token."),
        }
    }


    return opcode;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_two_plus_two() {
        assert_eq!(
            parse(vec![Token::Number(2), Token::Plus, Token::Number(2)]),
            vec![
                OP::Addition {
                    a: Literal::Number(2),
                    b: Literal::Number(2),
                },
            ]
        );
    }

    #[test]
    fn parse_multi_digit_addition() {
        assert_eq!(
            parse(vec![Token::Number(100), Token::Plus, Token::Number(5000)]),
            vec![
                OP::Addition {
                    a: Literal::Number(100),
                    b: Literal::Number(5000),
                },
            ]
        );
    }
}