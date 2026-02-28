use thiserror::Error;

/// Lexer result type for convenience
pub type LexerResult<T> = Result<T, LexerError>;

/// Possible lexer errors
#[derive(Error, Debug)]
pub enum LexerError {
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
}