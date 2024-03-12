use std::fmt;

#[derive(Debug, PartialEq)]
#[rustfmt::skip]
pub enum TokenKind {
    // Misc
    Space,
    Comma,

    // Numbers
    Int { str_val: String, val: u32 },

    // Math operations
    MathAdd, // +
    MathSub, // -
    MathMul, // *
    MathDiv, // /
    MathPow, // ^
    MathMod, // %

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

impl fmt::Display for TokenKind {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Int { str_val, .. } => write!(f, "{str_val}"),
            TokenKind::Space        => write!(f, " "),
            TokenKind::Comma        => write!(f, ","),
            TokenKind::MathAdd      => write!(f, "+"),
            TokenKind::MathSub      => write!(f, "-"),
            TokenKind::MathMul      => write!(f, "*"),
            TokenKind::MathDiv      => write!(f, "/"),
            TokenKind::MathPow      => write!(f, "^"),
            TokenKind::MathMod      => write!(f, "%"),
            TokenKind::LParen       => write!(f, "("),
            TokenKind::RParen       => write!(f, ")"),
            TokenKind::LSquiggly    => write!(f, "{{"),
            TokenKind::RSquiggly    => write!(f, "}}"),
            TokenKind::RngInclusive => write!(f, "..="),
            TokenKind::RngExclusive => write!(f, ".."),
            TokenKind::RngStep      => write!(f, "s:"),
            TokenKind::RngMutation  => write!(f, "m:"),
            TokenKind::RngMutArg    => write!(f, "@"),
                    
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
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
