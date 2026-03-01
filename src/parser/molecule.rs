use std::fmt::Display;
use fmtastic::Subscript;
use crate::element::Element;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::token::Token;
use crate::periodic_table::PERIODIC_TABLE;

#[derive(Debug, Clone, PartialEq)]
/// Chemical element with index (subscript)
pub struct ElementWithIndex {
	/// Element itself
	pub element: Element,
	/// Element index
	pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
/// Molecule of a chemical substance
pub struct Molecule {
	/// Molecule coefficient
	pub coefficient: isize,
	/// Elements in the molecule
	pub elements: Vec<ElementWithIndex>,
}

impl Molecule {
	/// Create a [`Molecule`] instance from tokens
	///
	/// # Example
	/// ```rust
	/// use chemistry_calculator::parser::{molecule::Molecule, token::{Lexer, Token}};
	///
	/// let input = "2HCl";
	/// let tokens = Lexer::new(input).tokenize().unwrap();
	/// let molecule = Molecule::from_tokens(tokens);
	/// ```
	pub fn from_tokens(tokens: Vec<Token>) -> ParserResult<Self> {
		let mut elements: Vec<ElementWithIndex> = Vec::new();
		let mut coefficient: isize = 1;
		let mut i = 0;
		while i < tokens.len() {
			match &tokens[i] {
				Token::Coefficient(c) => {
					coefficient = *c;
					i += 1;
				}
				Token::Element(symbol) => {
					let element = PERIODIC_TABLE
						.elements
						.iter()
						.find(|e| e.symbol == *symbol)
						.ok_or(ParserError::NonExistentElement(symbol.clone()))?
						.clone();

					let mut count: usize = 1;
					if i + 1 < tokens.len() {
						if let Token::Subscript(n) = &tokens[i + 1] {
							count = *n as usize;
							i += 1;
						}
					}

					elements.push(ElementWithIndex { element, index: count });
					i += 1;
				}
				Token::Subscript(_) => {
					return Err(ParserError::UnexpectedSubscript);
				}
				_ => {
					i += 1;
				}
			}
		}

		Ok(Self {
			coefficient,
			elements,
		})
	}
}

impl Display for Molecule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.coefficient != 1 {
			write!(f, "{}", self.coefficient)?;
		}

		for elem in &self.elements {
			write!(f, "{}", elem.element)?;
			if elem.index > 1 {
				write!(f, "{}", Subscript(elem.index))?;
			}
		}
		Ok(())
	}
}

impl Display for ElementWithIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.element)
	}
}