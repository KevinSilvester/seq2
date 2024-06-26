// use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    UnarySub,
    UnaryAdd,
}

impl Op {
    pub const LEFT_ASSOC: u8 = 0;
    pub const RIGHT_ASSOC: u8 = 1;

    pub fn precedence(&self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div | Op::Mod => 2,
            Op::Pow => 3,
            Op::UnaryAdd | Op::UnarySub => 4,
        }
    }

    pub fn associativity(&self) -> u8 {
        match self {
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => Self::LEFT_ASSOC,
            Op::UnaryAdd | Op::UnarySub => Self::RIGHT_ASSOC,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[rustfmt::skip]
pub enum TokenKind {
    // Misc
    Comma,

    // Numbers
    Int { value: i64 },

    // Math operations
    Math(Op),

    // Parentheses
    LParen,    // (
    RParen,    // )
    LSquiggly, // {
    RSquiggly, // }

    // Range
    RngInclusive, // ..=
    RngExclusive, // ..
    RngStep,      // s:
    RngMutation,  // m:
    RngMutArg,    // @
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
