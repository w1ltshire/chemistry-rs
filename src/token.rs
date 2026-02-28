use crate::element::Element;
use crate::error::{LexerError, LexerResult};

/// Enum representing possible chemical reaction expression tokens
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    /// Molecule coefficient
    Coefficient(isize),
    /// Element token
    Element(String),
    /// Subscript (index) token
    Subscript(isize),
    /// Plus (+) sign
    Plus,
    /// Left bracket sign
    LeftBracket,
    /// Right bracket sign
    RightBracket,
    /// Left square bracket sign
    LeftSquareBracket,
    /// Right square bracket sign
    RightSquareBracket,
    /// Equals (=) sign
    Equals,
    /// End of expression
    EOF
}

impl Token {
    pub fn from_string(kind: &str, _value: Option<String>) -> LexerResult<Token> {
        match kind {
            _ => Err(LexerError::InvalidToken(kind.to_string())),
        }
    }
}

pub fn tokenize(input: &str) -> LexerResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '+' => tokens.push(Token::Plus),
            '=' => tokens.push(Token::Equals),
            '(' => tokens.push(Token::LeftBracket),
            ')' => tokens.push(Token::RightBracket),
            '[' => tokens.push(Token::LeftSquareBracket),
            ']' => tokens.push(Token::RightSquareBracket),
            '1'..='9' => {
                let n: isize = std::iter::once(ch)
                    .chain(std::iter::from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()?;

                tokens.push(Token::Coefficient(n));
            },
            'A'..='Z' | 'a'..='z' => tokens.push(Token::Element(ch.to_string())),
            _ => return Err(LexerError::InvalidToken(String::from(ch)))
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}