use std::{io::Cursor, iter::Peekable, str::Chars};

use anyhow::{bail, Result};

use crate::errors::RangeToVecError;

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
    Inclusive { start: i32, end: i32 },
    Exclusive { start: i32, end: i32 },
}

#[derive(Debug, PartialEq)]
pub enum Paren {
    Open,
    Close,
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

    /// Parentheses ( )
    Parentheses(Paren),
}

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    ch: char,
    prev_ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
            ch: '\0',
            prev_ch: '\0',
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, RangeToVecError> {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            self.prev_ch = self.ch;
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
                    let operator = self.tokenize_operator(tokens.last())?;
                    tokens.push(operator);
                }
                '(' => todo!(),
                ')' => todo!(),
                '.' => todo!(),
                _ => return Err(RangeToVecError::InvalidToken(*ch, self.position)),
            }
        }
        Ok(tokens)
    }

    fn tokenize_operator(&mut self, last_token: Option<&Token>) -> Result<Token, RangeToVecError> {
        let op = match self.ch {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            '^' => Op::Pow,
            _ => unreachable!(),
        };

        // let mut number = Token::Number(0);
        // let mut is_negative = false;

        // match self.ch {
        //     '+' | '-' => {
        // self.input.next();
        // self.position += 1;

        // // let mut is_range_start = false;

        // if let Some(t @ Token::Comma) = last_token {
        //     is_negative = self.ch == '-';
        // } else {
        //     is_negative = self.ch == '-';
        // }

        // while let Some(ch @ ('0'..='9' | '.' | ' ')) = self.input.peek() {
        //     match ch {
        //         ' ' => {
        //             self.input.next();
        //             self.position += 1;
        //         }
        //         '.' => {
        //             todo!()
        //         }
        //         '0'..='9' => number = self.tokenize_numbers()?,
        //         _ => {
        //             todo!("Invalid token: {:?}", ch);
        //         }
        //     }
        // }

        // if is_negative {
        //     number = match number {
        //         Token::Number(n) => Token::Number(-n),
        //         _ => unreachable!(),
        //     };
        // }
        // return Ok(());
        //     }
        //     _ => {}
        // }
        Ok(Token::Operator(op))
    }

    fn is_arithmetic_expression(&mut self) -> bool {
        let mut is_arithmetic = false;

        while let Some(ch) = self.input.peek() {
            match ch {
                '+' | '-' | '*' | '/' | '^' => {
                    is_arithmetic = true;
                    break;
                }
                _ => {
                    self.input.next();
                    self.position += 1;
                }
            }
        }
        is_arithmetic
    }

    fn tokenize_numbers(&mut self) -> Result<Token, RangeToVecError> {
        let mut number = String::new();
        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            self.input.next();
            self.position += 1;
        }
        // tokens.push(Token::Number(number.parse::<i32>()?));
        Ok(Token::Number(number.parse::<u32>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
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
    fn test_parse_negative_numbers() {
        let mut lexer = Lexer::new("-123, 4, -10");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(Op::Sub),
                Token::Number(123),
                Token::Comma,
                Token::Number(4),
                Token::Comma,
                Token::Operator(Op::Sub),
                Token::Number(10)
            ]
        )
    }

    #[test]
    fn test_add_numbers() {
        let mut lexer = Lexer::new("123 + 2");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Operator(Op::Add),
                Token::Number(2)
            ]
        );
    }

    #[test]
    fn test_subtract_numbers() {
        let mut lexer = Lexer::new("123 - 2");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Operator(Op::Sub),
                Token::Number(2)
            ]
        );
    }

    #[test]
    fn test_everything() {
        let mut lexer =
            Lexer::new("123 - 2 + (-100 * 20), 1..5, -20..=-10, 5^2 / 5, 5^2 / (5 * 2)");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(123),
                Token::Operator(Op::Sub),
                Token::Number(2),
                Token::Operator(Op::Add),
                Token::Parentheses(Paren::Open),
                Token::Operator(Op::Add),
                Token::Number(100),
                Token::Operator(Op::Mul),
                Token::Number(20),
                Token::Parentheses(Paren::Close),
                Token::Comma,
                Token::Range(Rng::Exclusive { start: 1, end: 5 }),
                Token::Comma,
                Token::Range(Rng::Inclusive {
                    start: -20,
                    end: -10
                }),
                Token::Comma,
                Token::Number(5),
                Token::Operator(Op::Pow),
                Token::Number(2),
                Token::Operator(Op::Div),
                Token::Number(5),
                Token::Comma,
                Token::Number(5),
                Token::Operator(Op::Pow),
                Token::Number(2),
                Token::Operator(Op::Div),
                Token::Parentheses(Paren::Open),
                Token::Number(5),
                Token::Operator(Op::Mul),
                Token::Number(2),
                Token::Parentheses(Paren::Close),
            ]
        );
    }
}
