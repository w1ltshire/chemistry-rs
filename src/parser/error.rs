use thiserror::Error;

/// Lexer result type for convenience
pub type ParserResult<T> = Result<T, ParserError>;

/// Possible lexer errors
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid token: `{0}`")]
    InvalidToken(String),
    #[error("Value was None for a token `{0}` that requires a value")]
    NoneValue(String),
    #[error(transparent)]
    NumeralParseError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    FloatParseError(#[from] std::num::ParseFloatError),
    #[error("Iteration finished")]
    IterationFinished,
    #[error("Non-existent element `{0}`")]
    NonExistentElement(String),
    #[error("Unexpected subscript")]
    UnexpectedSubscript,
    #[error("Missing arrow")]
    MissingArrow,
}
