use std::fmt;

#[derive(Debug, PartialEq)]
#[rustfmt::skip]
pub enum Token {
    // Misc
    Space { pos: u16 },
    Comma { pos: u16 },

    // Numbers
    Int { start_pos: u16, end_pos: u16, str_val: String, val: u32 },

    // Math operations
    MathAdd { pos: u16 }, // +
    MathSub { pos: u16 }, // -
    MathMul { pos: u16 }, // *
    MathDiv { pos: u16 }, // /
    MathPow { pos: u16 }, // ^

    // Parentheses
    LParen    { pos: u16 }, // (
    RParen    { pos: u16 }, // )
    LSquiggly { pos: u16 }, // {
    RSquiggly { pos: u16 }, // }

    // Range
    RngInclusive { start_pos: u16, end_pos: u16 }, // ..=
    RngExclusive { start_pos: u16, end_pos: u16 }, // ..
    RngStep      { start_pos: u16, end_pos: u16 }, // s:
    RngMutation  { start_pos: u16, end_pos: u16 }, // m:
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Space { .. } => write!(f, " "),
            Token::Comma { .. } => write!(f, ","),
            Token::Int { str_val, .. } => write!(f, "{str_val}"),
            Token::MathAdd { .. } => write!(f, "+"),
            Token::MathSub { .. } => write!(f, "-"),
            Token::MathMul { .. } => write!(f, "*"),
            Token::MathDiv { .. } => write!(f, "/"),
            Token::MathPow { .. } => write!(f, "^"),
            Token::LParen { .. } => write!(f, "("),
            Token::RParen { .. } => write!(f, ")"),
            Token::LSquiggly { .. } => write!(f, "{{"),
            Token::RSquiggly { .. } => write!(f, "}}"),
            Token::RngInclusive { .. } => write!(f, "..="),
            Token::RngExclusive { .. } => write!(f, ".."),
            Token::RngStep { .. } => write!(f, "s:"),
            Token::RngMutation { .. } => write!(f, "m:"),
        }
    }
}
