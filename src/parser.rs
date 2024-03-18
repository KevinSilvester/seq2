use std::{iter::Peekable, slice::Iter};

use crate::{
    errors::ParserError,
    tokens::{Op, Span, Token, TokenKind},
};

#[derive(Debug, PartialEq)]
pub enum Node {
    Int {
        span: Span,
        value: i64,
    },
    MathExpr {
        span: Span,
        left: Box<Node>,
        right: Box<Node>,
        op: Op,
    },
    RangeExpr {
        span: Span,
        start: Box<Node>,
        end: Box<Node>,
        step: Option<Box<Node>>,
        mutation: Option<Box<Node>>,
    },
}

#[derive(Debug)]
pub struct Parser<'a> {
    input_chars: Vec<char>,
    tokens: Peekable<Iter<'a, Token>>,
    position: usize,
    current_token: Token,
    in_squiggly: bool,
    in_paren: bool,
    paren_depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input_chars: Vec<char>, tokens: &'a [Token]) -> Self {
        Self {
            input_chars,
            tokens: tokens.iter().peekable(),
            position: 0,
            current_token: tokens[0],
            in_squiggly: false,
            in_paren: false,
            paren_depth: 0,
        }
    }

    fn advance(&mut self) {
        self.tokens.next();
        self.position += 1;
    }

    fn prev_token(&self) -> Option<&Token> {
        self.tokens.clone().nth(self.position - 1)
    }

    fn advance_past_comma(&mut self) -> Result<(), ParserError> {
        let mut comma_count: u8 = 0;

        self.advance();

        while let Some(token) = self.tokens.peek() {
            match token.kind {
                TokenKind::Comma => {
                    self.current_token = **token;
                    self.advance();
                    comma_count += 1;

                    if comma_count > 1 {
                        return Err(ParserError::UnexpectedComma(
                            self.input_chars.clone(),
                            self.current_token.span,
                        ));
                    }
                }
                _ => break,
            }
        }

        self.current_token = match self.tokens.peek() {
            Some(token) => **token,
            None => return Ok(()),
        };

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ParserError> {
        let mut nodes = vec![];

        while let Some(token) = self.tokens.peek() {
            self.current_token = **token;
            let node = self.parse_t()?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    fn parse_t(&mut self) -> Result<Node, ParserError> {
        match self.current_token.kind {
            TokenKind::Int { .. } => {
                let int_node = self.parser_int()?;
                Ok(int_node)
            }

            // Error if the first token is a comma
            TokenKind::Comma => Err(ParserError::UnexpectedComma(
                self.input_chars.clone(),
                self.current_token.span,
            )),

            // Singular negative/positive numbers
            TokenKind::Math(op) => match op {
                Op::Add | Op::Sub => {
                    let int_node = self.parser_int()?;
                    Ok(int_node)
                }
                _ => Err(ParserError::UnexpectedMathOp(
                    self.input_chars.clone(),
                    self.current_token.span,
                )),
            },

            // Math expressions
            TokenKind::LParen => {
                let expr_node = self.parse_math_expr()?;
                Ok(expr_node)
            }

            _ => {
                todo!("Unexpected token: {:?}", self.current_token.kind)
            }
        }
    }

    fn parser_int(&mut self) -> Result<Node, ParserError> {
        let mut minus_count = 0;
        let span_start = self.current_token.span.start;

        // eat all '-' and '+' tokens before number
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                TokenKind::Math(Op::Add) => {
                    self.advance();
                }
                TokenKind::Math(Op::Sub) => {
                    self.advance();
                    minus_count += 1;
                }
                _ => break,
            }
        }

        let is_negative = minus_count % 2 != 0;

        // update the current token
        // returns error if there is no next token
        self.current_token = match self.tokens.peek() {
            Some(token) => **token,
            None => {
                return Err(ParserError::IncompleteInt(
                    self.input_chars.clone(),
                    self.current_token.span,
                ));
            }
        };

        match self.current_token.kind {
            TokenKind::Int { val } => {
                let int_node = match is_negative {
                    true => Node::Int {
                        span: Span::new(span_start, self.current_token.span.end),
                        value: -(val as i64),
                    },
                    false => Node::Int {
                        span: Span::new(span_start, self.current_token.span.end),
                        value: val as i64,
                    },
                };
                self.advance_past_comma()?;
                Ok(int_node)
            }
            _ => Err(ParserError::InvalidInt(
                self.input_chars.clone(),
                self.current_token.span,
            )),
        }
    }

    fn parse_math_expr(&mut self) -> Result<Node, ParserError> {
        self.paren_depth += 1;
        todo!()
    }
}
