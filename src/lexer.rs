/*
The goal of the lexer is to take in raw text, and provide a Vec[Token]
 */

use crate::util::Token;

pub(crate) fn lex (code: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut lines = code.lines().enumerate();
    while let Some((_i, line)) = lines.next() {

        let mut chars = line.chars().enumerate().peekable();
        while let Some((i, char)) = chars.next() {
            if char == ' ' {
                continue;
            }

            if char.is_numeric() {
                let mut number = char.to_string();

                while let Some((_i, next)) = chars.peek() {
                    if next.is_numeric() {
                        number += &*next.to_string();
                        chars.next();
                        continue;
                    } else if next == &'.' {
                        number += ".";
                        chars.next();
                    } else {
                        break;
                    };
                }

                tokens.push(Token::Number(number.parse::<i32>().unwrap()));
                continue;
            }

            if char == 'l' && chars.next() == Some((i+1, 'e')) && chars.next() == Some((i+2, 't')) {
                tokens.push(Token::Declaration);
                continue;
            }

            match char {
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Subtract),
                '/' => tokens.push(Token::Divide),
                '*' => tokens.push(Token::Multiply),
                '=' => tokens.push(Token::Equals),

                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                '{' => tokens.push(Token::LeftCurlyBrace),
                '}' => tokens.push(Token::RightCurlyBrace),

                ';' => tokens.push(Token::Delimiter),

                _ => tokens.push(Token::Identifier),
            }
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Token;

    #[test]
    fn return_empty_vec_on_empty_input() {
        assert_eq!(lex(""), vec![]);
    }

    #[test]
    fn lex_two_plus_two() {
        assert_eq!(lex("2 + 2"), vec![Token::Number(2), Token::Add, Token::Number(2)]);
    }

    #[test]
    fn lots_of_numbers() {
        assert_eq!(
            lex(
                "2147483647 + 2147483647 + 2147483647 + 2147483647 + 2147483647"
            ),
            vec![
                Token::Number(2147483647),
                Token::Add,
                Token::Number(2147483647),
                Token::Add,
                Token::Number(2147483647),
                Token::Add,
                Token::Number(2147483647),
                Token::Add,
                Token::Number(2147483647),
            ]
        );
    }

    #[test]
    fn lex_variables() {
        assert_eq!(
            lex("let a = 10; let b = a;"),
            vec![
                Token::Declaration, // let
                Token::Identifier,  // a
                Token::Equals,      // =
                Token::Number(10),  // 10
                Token::Delimiter,   // ;
                Token::Declaration, // let
                Token::Identifier,  // b
                Token::Equals,      // =
                Token::Identifier,  // a
                Token::Delimiter,   // ;
            ]
        );
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
        lex("(100)"),
        vec![
            Token::LeftParenthesis,
            Token::Number(100),
            Token::RightParenthesis,
        ]
        );
    }
}