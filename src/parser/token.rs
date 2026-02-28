use std::str::Chars;
use crate::parser::error::{LexerError, LexerResult};
use crate::parser::token::Token::Arrow;

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
}

impl<'a> Lexer<'a> {
	/// Create a new instance of [`Lexer`]
	pub fn new(input: &'a str) -> Self {
		Self {
			chars: input.chars(),
		}
	}

	/// Consume value and return tokens
	pub fn tokenize(mut self) -> LexerResult<Vec<Token>> {
		let mut tokens = Vec::new();
		loop {
			match self.next_token() {
				Ok(Token::Whitespace) => {}
				Ok(token) => tokens.push(token),
				Err(LexerError::IterationFinished) => break,
				Err(err) => return Err(err),
			}
		}
		Ok(tokens)
	}

	/// Parse the next token
	fn next_token(&mut self) -> LexerResult<Token> {
		let next_char = self.chars.next().ok_or(LexerError::IterationFinished)?;
		match next_char {
			' ' => Ok(Token::Whitespace),
			'+' => Ok(Token::Plus),
			'(' => Ok(Token::LeftParenthesis),
			')' => Ok(Token::RightParenthesis),
			'[' => Ok(Token::LeftBracket),
			']' => Ok(Token::RightBracket),
			'A'..='Z' | 'a'..='z' | '_' => Ok(Token::Element(next_char.to_string())),
			'1'..'9' => Ok(Token::Coefficient(next_char.to_string().parse()?)),
			'-' | '=' => {
				if let Some(next_char) = self.chars.next() {
					match next_char {
						'>' => Ok(Arrow(String::from("->"))),
						_ => Err(LexerError::InvalidToken(next_char.to_string()))
					}
				} else {
					Err(LexerError::IterationFinished)
				}
			}
			_ => Err(LexerError::InvalidToken(next_char.to_string()))
		}
	}
}