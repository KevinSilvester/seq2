use std::{iter::Peekable, str::Chars};

use crate::{
    errors::LexicalError,
    tokens::{Span, Token, TokenKind},
};

// type LexResult = Result<Vec<Token>, LexicalError>;
type TokenResult = Result<Token, LexicalError>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input_chars: Vec<char>,
    input: Peekable<Chars<'a>>,
    position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input_chars: input.chars().collect::<Vec<char>>(),
            input: input.chars().peekable(),
            position: 1,
            ch: '\0',
        }
    }

    fn next_char(&mut self) {
        self.input.next();
        self.position += 1;
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            self.ch = *ch;

            match *ch {
                ' ' => {
                    tokens.push(Token::new(
                        TokenKind::Space,
                        Span::new(self.position, self.position),
                    ));
                    self.next_char();
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenKind::Comma,
                        Span::new(self.position, self.position),
                    ));
                    self.next_char();
                }
                '@' => {
                    tokens.push(Token::new(
                        TokenKind::RngMutArg,
                        Span::new(self.position, self.position),
                    ));
                    self.next_char();
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
        self.next_char();
        Token::new(kind, Span::new(current_pos, current_pos))
    }

    fn tokenize_operator(&mut self) -> Token {
        let current_pos = self.position;
        let kind = match self.ch {
            '+' => TokenKind::MathAdd,
            '-' => TokenKind::MathSub,
            '*' => TokenKind::MathMul,
            '/' => TokenKind::MathDiv,
            '^' => TokenKind::MathPow,
            '%' => TokenKind::MathMod,
            _ => unreachable!(),
        };
        self.next_char();
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
                    self.next_char();
                }
                '=' => {
                    inclusive = true;
                    prev_ch = *ch;
                    self.next_char();
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
        self.next_char();

        if let Some(':') = self.input.peek() {
            let kind = match self.ch {
                's' => TokenKind::RngStep,
                'm' => TokenKind::RngMutation,
                _ => unreachable!(),
            };
            self.next_char();
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
        let mut str_val = String::new();
        let start_pos = self.position;

        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            str_val.push(*ch);
            self.next_char();
        }

        match number.parse::<u32>() {
            Ok(val) => Ok(Token::new(
                TokenKind::Int { val, str_val },
                Span::new(start_pos, self.position - 1),
            )),
            Err(_) => Err(LexicalError::MalformedNumber(
                self.input_chars.clone(),
                Span::new(start_pos, self.position),
            )),
        }
    }
}
