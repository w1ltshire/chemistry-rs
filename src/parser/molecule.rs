use crate::element::Element;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::token::Token;
use crate::periodic_table::PERIODIC_TABLE;

#[derive(Debug, Clone, PartialEq)]
pub struct ElementWithIndex {
	pub element: Element,
	pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
	pub coefficient: isize,
	pub elements: Vec<ElementWithIndex>,
}

impl Molecule {
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
