use crate::parser::error::{ParserError, ParserResult};
use crate::parser::token::Token::Arrow;
use std::str::Chars;

/// Possible chemical equation signs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Arrow (->, =>, ...) sign
    Arrow(String),
    /// Plus (+) sign
    Plus,
    /// Element (H, He, Cl, ...) symbol
    Element(String),
    /// Coefficient number (2H, ...)
    Coefficient(isize),
    /// Subscript number (H2O, O2, ...)
    Subscript(isize),
    /// Left parenthesis
    LeftParenthesis,
    /// Right parenthesis
    RightParenthesis,
    /// Left bracket
    LeftBracket,
    /// Right bracket
    RightBracket,
    /// End of equation
    EOF,
    /// Whitespace, shouldn't ever be returned by lexer
    Whitespace,
}

/// Structure representing a lexer
pub struct Lexer<'a> {
    chars: Chars<'a>,
    prev: Option<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new instance of [`Lexer`]
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            prev: None,
        }
    }

    /// Consume value and return tokens
    pub fn tokenize(mut self) -> ParserResult<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Ok(Token::Whitespace) => {}
                Ok(token) => tokens.push(token),
                Err(ParserError::IterationFinished) => break,
                Err(err) => return Err(err),
            }
        }
        Ok(tokens)
    }

    /// Parse the next token
    fn next_token(&mut self) -> ParserResult<Token> {
        let next_char = self.chars.next().ok_or(ParserError::IterationFinished)?;
        let token = match next_char {
            ' ' => Ok(Token::Whitespace),
            '+' => Ok(Token::Plus),
            '(' => Ok(Token::LeftParenthesis),
            ')' => Ok(Token::RightParenthesis),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            'A'..='Z' => {
                let mut sym = next_char.to_string();
                if let Some(&c) = self.chars.clone().peekable().peek() {
                    if c.is_ascii_lowercase() {
                        // consume the lowercase letter
                        let c = self.chars.next().unwrap();
                        sym.push(c);
                    }
                }
                Ok(Token::Element(sym))
            }
            '1'..='9' => {
                let mut num = next_char.to_digit(10).unwrap() as isize;
                while let Some(&c) = self.chars.clone().peekable().peek() {
                    if c.is_ascii_digit() {
                        self.chars.next();
                        num = num * 10 + c.to_digit(10).unwrap() as isize;
                    } else {
                        break;
                    }
                }

                return if let Some(prev) = self.prev.clone() {
                    match prev {
                        Token::Element(_) | Token::RightParenthesis | Token::RightBracket => {
                            Ok(Token::Subscript(num))
                        }
                        _ => Ok(Token::Coefficient(num)),
                    }
                } else {
                    // start of line so it's a coefficient
                    Ok(Token::Coefficient(num))
                };
            }
            '-' | '=' => {
                if let Some(next_char) = self.chars.next() {
                    match next_char {
                        '>' => Ok(Arrow(String::from("->"))),
                        _ => Err(ParserError::InvalidToken(next_char.to_string())),
                    }
                } else {
                    Err(ParserError::IterationFinished)
                }
            }
            _ => Err(ParserError::InvalidToken(next_char.to_string())),
        }?;
        self.prev = Some(token.clone());
        Ok(token)
    }
}
