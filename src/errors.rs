use thiserror::Error;

#[derive(Error, Debug)]
pub enum RangeToVecError {
    #[error("Invalid token '{0}' at position {1}")]
    InvalidToken(char, usize),

    #[error("Could not parse number '{0}'")]
    ParseIntError(#[from] std::num::ParseIntError),
}
