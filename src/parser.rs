use std::{iter::Peekable, slice::Iter};

use crate::{
    errors::ParserError,
    tokens::{Op, Span, Token, TokenKind},
};

/// NOICE!
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠠⠤⠶⠞⢻⣿⡿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⢀⣠⣤⣤⣴⣶⣄⠀⢸⣿⠇⠻⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⠀⠀⠰⠛⠛⠛⠻⠿⠿⣿⡇⠈⠉⠀⠀⠈⠻⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⣄⡀⢹⣿
/// ⣿⣿⣿⣿⣿⣿⢿⢏⡜⢱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠾⢿⣿⠻⣿⣿⣿
/// ⣿⣿⣿⣿⣿⡿⢸⡞⣠⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠀⠀⠹⡍
/// ⣉⠙⠻⣿⣹⡇⡞⢰⡟⠀⣠⠤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹
/// ⠈⡹⣶⢆⣿⢱⡇⢸⡷⠋⣠⣤⡈⢇⠀⠀⠀⠀⠀⠀⠀⢀⡤⠄⢠⡀⠀⠀⠀⠈
/// ⠀⣇⣿⢸⣿⠸⣷⠀⢧⣾⠋⠈⠻⣾⣦⠀⠀⠀⠀⠀⣴⠋⢀⣦⠀⢿⠀⠀⠀⢀
/// ⡀⠈⣿⠘⢿⠄⠈⢀⠸⡏⠀⠀⢰⡇⡜⠀⠀⠀⠀⠀⠁⠀⠈⢸⠈⠀⠀⠀⠀⡼
/// ⣿⣷⣿⠀⠀⠀⠀⡌⠀⢧⣀⡴⠛⢁⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡇
/// ⣿⣿⣿⡇⠀⠀⠰⢰⠀⠀⠙⠃⢀⡾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⡇
/// ⣿⣿⣿⣷⠀⠀⠀⡸⠀⠀⠀⣠⣿⣿⣶⣤⣤⣀⡀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⡇
/// ⣿⣿⣿⠏⠀⠀⢀⡇⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⡞⠋⢸⣿⣿⣿⣿⡇
/// ⣿⡿⠃⠀⠐⠶⣿⡿⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣞⢻⣿⣿⣿⣿⡇
pub const MAX_PAREN_DEPTH: usize = 69;

#[derive(Debug, PartialEq)]
pub enum Node {
    Int {
        span: Span,
        value: i64,
    },
    MathExpr {
        negated: bool,
        span: Span,
        rpn: Vec<Token>,
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

    fn check_unmatched_paren(&self) -> Result<(), ParserError> {
        let mut stack = vec![];

        for token in self.tokens.clone() {
            match token.kind {
                TokenKind::LParen => stack.push(token.span),
                TokenKind::RParen => {
                    if stack.pop().is_none() {
                        return Err(ParserError::UnmatchedParen(
                            self.input_chars.clone(),
                            token.span,
                        ));
                    }
                }
                TokenKind::Math(_) | TokenKind::Int { .. } => {}
                _ => break,
            }
        }

        if let Some(span) = stack.pop() {
            return Err(ParserError::UnmatchedParen(self.input_chars.clone(), span));
        }

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
            TokenKind::Int { value: val } => {
                let int_node = match is_negative {
                    true => Node::Int {
                        span: Span::new(span_start, self.current_token.span.end),
                        value: -val,
                    },
                    false => Node::Int {
                        span: Span::new(span_start, self.current_token.span.end),
                        value: val,
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

    // TODO: Switch to use shunting yard algorithm
    fn parse_math_expr(&mut self) -> Result<Node, ParserError> {
        self.check_unmatched_paren()?;
        self.in_paren = true;

        let span_start = self.current_token.span.start;
        let mut start_of_expr = true;
        let mut paren_depth = 0;
        let mut ouput_queue = vec![];
        let mut operator_stack = vec![];

        // Advance the cursor
        self.advance();

        // while let Some(token) = self.tokens.peek() {
        //     self.current_token = **token;

        //     match token.kind {
        //         TokenKind::LParen => {
        //             self.advance();
        //             paren_depth += 1;
        //             if paren_depth > 1 {
        //                 operator_stack.push(self.current_token);
        //                 start_of_expr = true;
        //             }
        //         }

        //         // Singular negative/positive numbers at the start of the expression/parenthesis
        //         TokenKind::Math(op) if start_of_expr => match op {
        //             Op::Add | Op::Sub => {
        //                 let int_token = match self.parser_int()? {
        //                     Node::Int { value, .. } => TokenKind::Int { value },
        //                     _ => unreachable!(),
        //                 };
        //                 ouput_queue.push(int_token);
        //             }
        //             _ => {
        //                 return Err(ParserError::UnexpectedMathOp(
        //                     self.input_chars.clone(),
        //                     self.current_token.span,
        //                 ))
        //             }
        //         },

        //         TokenKind::Math(_) if !start_of_expr => {
        //             operator_stack.push(self.current_token);
        //         }
        //         _ => {
        //             let node = self.parse_t()?;
        //         }

        //         TokenKind::RParen => {
        //             self.advance();
        //             paren_depth -= 1;
        //         }
        //     }

        //     start_of_expr = false;
        // }

        self.infix_to_postfix(span_start, &mut ouput_queue, &mut operator_stack)?;

        todo!("Implement shunting yard algorithm")
    }

    // A recursive infix to postfix translator based on shunting yard algorithm
    fn infix_to_postfix(
        &mut self,
        start: usize,
        ouput_queue: &mut Vec<Token>,
        operator_stack: &mut Vec<Token>,
    ) -> Result<(), ParserError> {
        self.paren_depth += 1;
        self.advance();
        let mut token_count = 0; // keeps track of tokens in parenthesis
        let mut is_start = true; // whether position of the cursor is at the start of a new number of nested maths epxr. (For parsing unary operators)

        if self.paren_depth > MAX_PAREN_DEPTH {
            return Err(ParserError::TooManyParen(
                self.input_chars.clone(),
                Span::new(start, self.current_token.span.end),
            ));
        }

        while let Some(token) = self.tokens.peek() {
            self.current_token = **token;

            match self.current_token.kind {
                // End of math expression
                TokenKind::RParen => {
                    self.advance();
                    self.paren_depth -= 1;
                    if self.paren_depth > 0 {
                        todo!("Pop the opertator stack")
                    }
                    break;
                }

                // Nested math expression
                TokenKind::LParen => self.infix_to_postfix(start, ouput_queue, operator_stack)?,

                // Numbers
                TokenKind::Int { .. } => {}

                // Singular negative/positive numbers at the start of the expression/parenthesis
                TokenKind::Math(op) if is_start => match op {
                    Op::Add | Op::Sub => {
                        let int_token = match self.parser_int()? {
                            Node::Int { value, span } => Token::new(TokenKind::Int { value }, span),
                            _ => unreachable!(),
                        };
                        ouput_queue.push(int_token);
                    }
                    _ => {
                        return Err(ParserError::UnexpectedMathOp(
                            self.input_chars.clone(),
                            self.current_token.span,
                        ))
                    }
                },

                // Math operators + negated numbers/nested math expression
                TokenKind::Math(op) => {}

                // Any other token is invalid syntax
                _ => {
                    return Err(ParserError::IncompleteMathExpr(
                        self.input_chars.clone(),
                        Span::new(start, token.span.end),
                    ))
                }
            }
            is_start = false;
        }

        if token_count == 0 {
            return Err(ParserError::EmptyParen(
                self.input_chars.clone(),
                Span::new(start, self.current_token.span.end + 1),
            ));
        }

        Ok(())
    }
}
