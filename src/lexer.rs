use std::{iter::Peekable, num::IntErrorKind, str::Chars};

use crate::{
    errors::LexicalError,
    tokens::{Op, Span, Token, TokenKind},
};

type LexResult = Result<Vec<Token>, LexicalError>;
type TokenResult = Result<Token, LexicalError>;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input_chars: Vec<char>,
    input: Peekable<Chars<'a>>,
    position: usize,
    ch: char,
    in_squiggly: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input_chars: input.chars().collect::<Vec<char>>(),
            input: input.chars().peekable(),
            position: 1,
            ch: '\0',
            in_squiggly: false,
        }
    }

    fn advance(&mut self) {
        self.input.next();
        self.position += 1;
    }

    pub fn lex(&mut self) -> LexResult {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            self.ch = *ch;

            match *ch {
                ' ' => {
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenKind::Comma,
                        Span::new(self.position, self.position),
                    ));
                    self.advance();
                }
                '@' => {
                    if !self.in_squiggly {
                        return Err(LexicalError::MisplacedRngSyntax(
                            self.input_chars.clone(),
                            Span::new(self.position, self.position),
                        ));
                    }
                    tokens.push(Token::new(
                        TokenKind::RngMutArg,
                        Span::new(self.position, self.position),
                    ));
                    self.advance();
                }
                '0'..='9' => {
                    let number = self.tokenize_numbers()?;
                    tokens.push(number);
                }
                '.' => {
                    let range = self.tokenize_range()?;
                    tokens.push(range);
                }
                's' | 'm' => {
                    let range_arg = self.tokenize_range_arg()?;
                    tokens.push(range_arg);
                }
                '+' | '-' | '*' | '/' | '^' | '%' => {
                    let operator = self.tokenize_operator();
                    tokens.push(operator);
                }
                '(' | ')' | '{' | '}' => {
                    let paren = self.tokenize_parenteses();
                    tokens.push(paren);
                }
                '\0' => break,
                _ => {
                    return Err(LexicalError::InvalidToken(
                        self.input_chars.clone(),
                        Span::new(self.position, self.position),
                    ));
                }
            }
        }

        Ok(tokens)
    }

    fn tokenize_parenteses(&mut self) -> Token {
        let current_pos = self.position;
        let kind = match self.ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LSquiggly,
            '}' => TokenKind::RSquiggly,
            _ => unreachable!(),
        };
        if kind == TokenKind::LSquiggly {
            self.in_squiggly = true;
        } else if kind == TokenKind::RSquiggly {
            self.in_squiggly = false;
        }
        self.advance();
        Token::new(kind, Span::new(current_pos, current_pos))
    }

    fn tokenize_operator(&mut self) -> Token {
        let current_pos = self.position;
        let kind = match self.ch {
            '+' => TokenKind::Math(Op::Add),
            '-' => TokenKind::Math(Op::Sub),
            '*' => TokenKind::Math(Op::Mul),
            '/' => TokenKind::Math(Op::Div),
            '^' => TokenKind::Math(Op::Pow),
            '%' => TokenKind::Math(Op::Mod),
            _ => unreachable!(),
        };
        self.advance();
        Token::new(kind, Span::new(current_pos, current_pos))
    }

    fn tokenize_range(&mut self) -> TokenResult {
        let mut dot_count = 0;
        let mut inclusive = false;
        let start_pos = self.position;
        let mut prev_ch = self.ch;

        while let Some(ch @ ('.' | '=')) = self.input.peek() {
            match *ch {
                '.' => {
                    if prev_ch == '=' {
                        return Err(LexicalError::UnexpectedEqual(
                            self.input_chars.clone(),
                            Span::new(start_pos, self.position),
                        ));
                    }

                    dot_count += 1;

                    prev_ch = *ch;
                    self.advance();
                }
                '=' => {
                    inclusive = true;
                    prev_ch = *ch;
                    self.advance();
                }
                _ => {}
            }
        }

        if dot_count != 2 {
            return Err(LexicalError::InvalidRange(
                self.input_chars.clone(),
                Span::new(start_pos, self.position - 1),
            ));
        }

        let kind = match inclusive {
            true => TokenKind::RngInclusive,
            false => TokenKind::RngExclusive,
        };

        Ok(Token::new(kind, Span::new(start_pos, self.position - 1)))
    }

    fn tokenize_range_arg(&mut self) -> TokenResult {
        let start_pos = self.position;
        self.advance();

        if !self.in_squiggly {
            return Err(LexicalError::MisplacedRngSyntax(
                self.input_chars.clone(),
                Span::new(start_pos, self.position - 1),
            ));
        }

        if let Some(':') = self.input.peek() {
            let kind = match self.ch {
                's' => TokenKind::RngStep,
                'm' => TokenKind::RngMutation,
                _ => unreachable!(),
            };
            self.advance();
            Ok(Token::new(kind, Span::new(start_pos, self.position - 1)))
        } else {
            Err(LexicalError::MissingColon(
                self.input_chars.clone(),
                Span::new(start_pos, self.position - 1),
            ))
        }
    }

    fn tokenize_numbers(&mut self) -> TokenResult {
        let mut number = String::new();
        let start_pos = self.position;

        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            self.advance();
        }

        match number.parse::<i64>() {
            Ok(val) => Ok(Token::new(
                TokenKind::Int { value: val },
                Span::new(start_pos, self.position - 1),
            )),
            Err(e) if e.kind() == &IntErrorKind::PosOverflow => Err(LexicalError::NumberTooLarge(
                self.input_chars.clone(),
                Span::new(start_pos, self.position - 1),
            )),
            Err(_) => Err(LexicalError::MalformedNumber(
                self.input_chars.clone(),
                Span::new(start_pos, self.position - 1),
            )),
        }
    }
}
