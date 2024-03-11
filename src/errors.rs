use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Invalid token '{0}' at position {1}")]
    InvalidToken(char, u16),

    #[error("Expected a trailing ':' after '{0}' at position {1}")]
    MissingColon(char, u16),

    #[error("Missing comma at possition {0}")]
    MissingComma(u16),

    #[error("Unexpected '=' at position {0}")]
    UnexpectedEqual(u16),

    #[error("Invalid range starting at position {0}-{1}")]
    InvalidRange(u16, u16),

    #[error("Malformed number starting at position {0}-1")]
    MalformedNumber(String, u16, u16),
}
