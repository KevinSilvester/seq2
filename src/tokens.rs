// use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[rustfmt::skip]
pub enum TokenKind {
    // Misc
    // Space,
    // START,
    // END,
    Comma,

    // Numbers
    Int { val: u32 },

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

// impl fmt::Display for TokenKind {
//     #[rustfmt::skip]
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TokenKind::Int { str_val, .. } => write!(f, "{str_val}"),
//             // TokenKind::Space        => write!(f, " "),
//             TokenKind::Comma        => write!(f, ","),
//             TokenKind::MathAdd      => write!(f, "+"),
//             TokenKind::MathSub      => write!(f, "-"),
//             TokenKind::MathMul      => write!(f, "*"),
//             TokenKind::MathDiv      => write!(f, "/"),
//             TokenKind::MathPow      => write!(f, "^"),
//             TokenKind::MathMod      => write!(f, "%"),
//             TokenKind::LParen       => write!(f, "("),
//             TokenKind::RParen       => write!(f, ")"),
//             TokenKind::LSquiggly    => write!(f, "{{"),
//             TokenKind::RSquiggly    => write!(f, "}}"),
//             TokenKind::RngInclusive => write!(f, "..="),
//             TokenKind::RngExclusive => write!(f, ".."),
//             TokenKind::RngStep      => write!(f, "s:"),
//             TokenKind::RngMutation  => write!(f, "m:"),
//             TokenKind::RngMutArg    => write!(f, "@"),
//         }
//     }
// }

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

// #[derive(Debug)]
// pub struct TokenIter<'a> {
//     tokens: &'a [Token],
//     index: usize,
// }

// impl<'a> TokenIter<'a> {
//     pub fn new(tokens: &'a [Token]) -> Self {
//         Self { tokens, index: 0 }
//     }
// }

// impl<'a> Iterator for TokenIter<'a> {
//     type Item = &'a Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.tokens.is_empty() {
//             return None;
//         }

//         let token = &self.tokens[self.index];
//         self.index += 1;
//         Some(token)
//     }
// }

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }

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
