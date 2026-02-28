use crate::error::{LexerError, LexerResult};

/// Enum representing possible chemical reaction expression tokens
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    Coefficient(isize),
    /// Plus operator token
    Plus,
    /// Left bracket
    LeftBracket,
    /// Right bracket
    RightBracket,
}

impl Token {
    pub fn from_string(kind: &str, value: Option<String>) -> LexerResult<Token> {
        match kind {
            _ => Err(LexerError::InvalidToken(kind.to_string())),
        }
    }
}
