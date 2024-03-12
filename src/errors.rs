use std::fmt;

use anstyle::{Effects, RgbColor};
use indoc::formatdoc;

use crate::tokens::Span;

trait FancyError {
    fn error_ctx(&self) -> (&Vec<char>, Span);
    fn error_msg(&self) -> String;

    fn construct_error(&self) -> String {
        let (input, span) = self.error_ctx();
        let msg = self.error_msg();
        let red = RgbColor(235, 66, 66).on_default() | Effects::BOLD;
        let turquiouse = RgbColor(64, 224, 208).on_default() | Effects::BOLD;

        let before_err: String = input[0..(span.start - 1)].iter().collect();
        let after_err: String = input[span.end..].iter().collect();
        let err: String = input[(span.start - 1)..span.end].iter().collect();

        let error_msg = formatdoc! {"
            --> {red}ERROR{red:#}: {msg}
            |
            | {before_err}{red}{err}{red:#}{after_err}
            |
            | = {turquiouse}HINT{turquiouse:#}: touch grass ;)
        "};
        error_msg
    }
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken(Vec<char>, Span),
    MissingColon(Vec<char>, Span),
    InvalidRange(Vec<char>, Span),
    UnexpectedEqual(Vec<char>, Span),
    MalformedNumber(Vec<char>, Span),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvalidToken(_, _)
            | LexicalError::MissingColon(_, _)
            | LexicalError::UnexpectedEqual(_, _)
            | LexicalError::InvalidRange(_, _)
            | LexicalError::MalformedNumber(_, _) => write!(f, "{}", self.construct_error()),
        }
    }
}

impl FancyError for LexicalError {
    fn error_ctx(&self) -> (&Vec<char>, Span) {
        match self {
            LexicalError::InvalidToken(input, span)
            | LexicalError::MissingColon(input, span)
            | LexicalError::UnexpectedEqual(input, span)
            | LexicalError::InvalidRange(input, span)
            | LexicalError::MalformedNumber(input, span) => (input, *span),
        }
    }

    fn error_msg(&self) -> String {
        match self {
            LexicalError::InvalidToken(_, span) => {
                format!("Invalid token at position {}", span.start)
            }
            LexicalError::MissingColon(input, span) => {
                format!(
                    "Expected a trailing ':' after '{}' at position {}",
                    input[span.start - 1],
                    span.start
                )
            }
            LexicalError::UnexpectedEqual(_, span) => {
                format!("Unexpected '=' at position {}", span.start)
            }
            LexicalError::InvalidRange(_, span) => {
                format!(
                    "Invalid range syntax at position {} - {}",
                    span.start, span.end
                )
            }
            LexicalError::MalformedNumber(_, span) => {
                format!(
                    "Malformed number starting at position {} - {}",
                    span.start, span.end
                )
            }
        }
    }
}
