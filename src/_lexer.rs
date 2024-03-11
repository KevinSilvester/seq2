use std::{iter::Peekable, str::Chars};

use anyhow::{bail, Result};

use crate::errors::LexerError;

#[derive(Debug, PartialEq)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, PartialEq)]
pub enum Rng {
    Inclusive,
    Exclusive,
    Step,
    Mutation,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    /// numbers
    Number(u32),

    /// Comma separator
    Comma,

    /// 1..5
    Range(Rng),

    /// Math operators
    Operator(Op),

    /// Parentheses ( ) { }
    LParen,
    RParen,
    LSquiggly,
    RSquiggly,
}

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: u16,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
            ch: '\0',
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            self.ch = *ch;

            match ch {
                // Skip whitespace
                ' ' => {
                    self.input.next();
                    self.position += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.input.next();
                    self.position += 1;
                }
                '0'..='9' => {
                    let number = self.tokenize_numbers()?;
                    tokens.push(number);
                }
                '+' | '-' | '*' | '/' | '^' => {
                    let operator = self.tokenize_operator()?;
                    tokens.push(operator);
                }
                '(' | ')' | '{' | '}' => {
                    let paren = self.tokenize_parentheses();
                    tokens.push(paren);
                }
                '.' => {
                    let range = self.tokenize_range()?;
                    tokens.push(range);
                }
                's' | 'm' => {
                    let mut range = String::new();
                    let mut dot_count = 0;
                    let mut inclusive = false;
                }
                _ => return Err(LexerError::InvalidToken(*ch, self.position)),
            }
        }
        Ok(tokens)
    }

    fn tokenize_operator(&mut self) -> Result<Token, LexerError> {
        let op = match self.ch {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            '^' => Op::Pow,
            _ => unreachable!(),
        };

        self.input.next();
        self.position += 1;

        Ok(Token::Operator(op))
    }

    fn tokenize_parentheses(&mut self) -> Token {
        let paren = match self.ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LSquiggly,
            '}' => Token::RSquiggly,
            _ => unreachable!(),
        };
        self.input.next();
        self.position += 1;
        paren
    }

    fn tokenize_numbers(&mut self) -> Result<Token, LexerError> {
        let mut number = String::new();
        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            self.input.next();
            self.position += 1;
        }
        Ok(Token::Number(number.parse::<u32>()?))
    }

    fn tokenize_range(&mut self) -> Result<Token, LexerError> {
        let mut range = String::new();
        let mut dot_count = 0;
        let mut inclusive = false;

        while let Some(ch @ ('.' | '=')) = self.input.peek() {
            if *ch == '.' {
                dot_count += 1;
            } else if *ch == '=' {
                inclusive = true;
            }
            range.push(*ch);
            self.input.next();
            self.position += 1;
        }

        if dot_count != 2 {
            return Err(LexerError::InvalidRange(range, self.position - dot_count));
        }

        if inclusive {
            Ok(Token::Range(Rng::Inclusive))
        } else {
            Ok(Token::Range(Rng::Exclusive))
        }
    }

    fn tokenize_range_args(&mut self) -> Result<Token, LexerError> {
        let mut range = String::new();
        let mut dot_count = 0;
        let mut inclusive = false;
        while let Some(ch @ ('.' | '=')) = self.input.peek() {
            if *ch == '.' {
                dot_count += 1;
            } else if *ch == '=' {
                inclusive = true;
            }
            range.push(*ch);
            self.input.next();
            self.position += 1;
        }
        if dot_count != 2 {
            return Err(LexerError::InvalidRange(range, self.position - dot_count));
        }
        if inclusive {
            Ok(Token::Range(Rng::Inclusive))
        } else {
            Ok(Token::Range(Rng::Exclusive))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_tokenize_numbers() {
        let mut lexer = Lexer::new("123, 4, 10");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Comma,
                Token::Number(4),
                Token::Comma,
                Token::Number(10)
            ]
        )
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new("1 + 2 - 3 * 4 / 5 ^ 6");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Operator(Op::Add),
                Token::Number(2),
                Token::Operator(Op::Sub),
                Token::Number(3),
                Token::Operator(Op::Mul),
                Token::Number(4),
                Token::Operator(Op::Div),
                Token::Number(5),
                Token::Operator(Op::Pow),
                Token::Number(6)
            ]
        )
    }

    #[test]
    fn test_tokenize_parentheses() {
        let mut lexer = Lexer::new("(1 + 2) * (3 - 4)");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Number(1),
                Token::Operator(Op::Add),
                Token::Number(2),
                Token::RParen,
                Token::Operator(Op::Mul),
                Token::LParen,
                Token::Number(3),
                Token::Operator(Op::Sub),
                Token::Number(4),
                Token::RParen
            ]
        );

        let mut lexer = Lexer::new("{1 + 2} * {3 - 4}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly,
                Token::Number(1),
                Token::Operator(Op::Add),
                Token::Number(2),
                Token::RSquiggly,
                Token::Operator(Op::Mul),
                Token::LSquiggly,
                Token::Number(3),
                Token::Operator(Op::Sub),
                Token::Number(4),
                Token::RSquiggly
            ]
        )
    }

    #[test]
    fn test_tokenize_invalid_token() {
        let mut lexer = Lexer::new("1 + 2 & 3");
        let result = lexer.lex();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid token '&' at position 6"
        )
    }

    #[test]
    fn test_toknize_range() {
        let mut lexer = Lexer::new("{1..5}, {1..=-5}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly,
                Token::Number(1),
                Token::Range(Rng::Exclusive),
                Token::Number(5),
                Token::RSquiggly,
                Token::Comma,
                Token::LSquiggly,
                Token::Number(1),
                Token::Range(Rng::Inclusive),
                Token::Operator(Op::Sub),
                Token::Number(5),
                Token::RSquiggly,
            ]
        )
    }

    #[test]
    fn test_tokenize_invalid_range() {
        let mut lexer = Lexer::new("{1...5}, {1..=5}");
        let result = lexer.lex();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid range '...' at position 2"
        )
    }
}
