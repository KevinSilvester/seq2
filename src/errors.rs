use std::fmt;

use anstyle::{Color, Effects, RgbColor};
use indoc::formatdoc;

use crate::tokens::Span;

const RED: RgbColor = RgbColor(235, 66, 66);
const WHITE: RgbColor = RgbColor(255, 255, 255);
const CYAN: RgbColor = RgbColor(64, 224, 208);
const BLUE: RgbColor = RgbColor(66, 117, 235);

trait FancyError {
    fn error_ctx(&self) -> (&Vec<char>, Span);
    fn error_msg(&self) -> String;

    fn construct_error(&self) -> String {
        let (input, span) = self.error_ctx();
        let msg = self.error_msg();
        let red = RED.on_default() | Effects::BOLD;
        let white_on_red = WHITE.on(Color::from(RED)) | Effects::BOLD;
        let cyan = CYAN.on_default() | Effects::BOLD;

        let before_err: String = input[0..(span.start - 1)].iter().collect();
        let after_err: String = input[span.end..].iter().collect();
        let err: String = input[(span.start - 1)..span.end].iter().collect();

        let error_msg = formatdoc! {"
            --> {red}ERROR{red:#}: {msg}
            |
            | {before_err}{white_on_red}{err}{white_on_red:#}{after_err}
            |
            | = {cyan}HINT{cyan:#}: touch grass ;)
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
    MisplacedRngSyntax(Vec<char>, Span),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvalidToken(_, _)
            | LexicalError::MissingColon(_, _)
            | LexicalError::UnexpectedEqual(_, _)
            | LexicalError::InvalidRange(_, _)
            | LexicalError::MalformedNumber(_, _)
            | LexicalError::MisplacedRngSyntax(_, _) => write!(f, "{}", self.construct_error()),
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
            | LexicalError::MalformedNumber(input, span)
            | LexicalError::MisplacedRngSyntax(input, span) => (input, *span),
        }
    }

    fn error_msg(&self) -> String {
        let blue = BLUE.on_default() | Effects::BOLD;

        match self {
            LexicalError::InvalidToken(_, span) => {
                format!("{blue}@ position {}{blue:#} - Invalid token", span.start)
            }
            LexicalError::MissingColon(input, span) => {
                format!(
                    "{blue}@ position {}{blue:#} - Expected a trailing ':' after '{}'",
                    span.start,
                    input[span.start - 1],
                )
            }
            LexicalError::UnexpectedEqual(_, span) => {
                format!("{blue}@ position {}{blue:#} - Unexpected '='", span.start)
            }
            LexicalError::InvalidRange(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} - Invalid range syntax",
                    span.start, span.end
                )
            }
            LexicalError::MalformedNumber(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} - Malformed number",
                    span.start, span.end
                )
            }
            LexicalError::MisplacedRngSyntax(input, span) => {
                format!(
                    "{blue}@ position {}{blue:#} - Character '{}' can only be used when defining number ranges",
                    span.start,
                    input[span.start - 1],
                )
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ParserError {
    UnexpectedComma(Vec<char>, Span),
    UnexpectedMathOp(Vec<char>, Span),
    IncompleteInt(Vec<char>, Span),
    InvalidInt(Vec<char>, Span),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedComma(_, _)
            | ParserError::UnexpectedMathOp(_, _)
            | ParserError::InvalidInt(_, _)
            | ParserError::IncompleteInt(_, _) => {
                write!(f, "{}", self.construct_error())
            }
        }
    }
}

impl FancyError for ParserError {
    fn error_ctx(&self) -> (&Vec<char>, Span) {
        match self {
            ParserError::UnexpectedComma(input, span)
            | ParserError::UnexpectedMathOp(input, span)
            | ParserError::InvalidInt(input, span)
            | ParserError::IncompleteInt(input, span) => (input, *span),
        }
    }
    fn error_msg(&self) -> String {
        let blue = BLUE.on_default() | Effects::BOLD;
        match self {
            ParserError::UnexpectedComma(_, span) => {
                format!("{blue}@ position {}{blue:#} - Unexpected comma", span.start)
            }
            ParserError::UnexpectedMathOp(input, span) => {
                format!(
                    "{blue}@ position {}{blue:#} - Unexpected math operator '{}'",
                    span.start,
                    input[span.start - 1]
                )
            }
            ParserError::IncompleteInt(input, span) => {
                format!(
                    "{blue}@ position {}{blue:#} - Expected a number after the math operator '{}'",
                    span.start,
                    input[span.start - 1]
                )
            }
            ParserError::InvalidInt(input, span) => {
                format!(
                    "{blue}@ position {}{blue:#} - Expected a number, found '{}'",
                    span.start,
                    input[span.start - 1]
                )
            }
        }
    }
}
