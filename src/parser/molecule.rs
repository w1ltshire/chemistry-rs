use std::collections::BTreeMap;
use std::fmt::Display;
use crate::element::Element;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::token::Token;
use crate::periodic_table::PERIODIC_TABLE;

#[derive(Debug, Clone, PartialEq)]
pub enum MoleculePart {
	Element { element: Element, index: usize },
	Group { parts: Vec<MoleculePart>, index: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
	pub coefficient: isize,
	pub parts: Vec<MoleculePart>,
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
		let mut i = 0;
		let mut coefficient: isize = 1;

		let resolve_element = |sym: &String| -> ParserResult<Element> {
			PERIODIC_TABLE
				.elements
				.iter()
				.find(|e| e.symbol == *sym)
				.ok_or(ParserError::NonExistentElement(sym.clone()))
				.map(|e| e.clone())
		};

		fn parse_parts(
			tokens: &[Token],
			mut i: usize,
			resolve_element: &dyn Fn(&String) -> ParserResult<Element>,
		) -> ParserResult<(Vec<MoleculePart>, usize)> {
			let mut parts: Vec<MoleculePart> = Vec::new();
			while i < tokens.len() {
				match &tokens[i] {
					Token::Element(sym) => {
						let elem = resolve_element(sym)?;
						let mut idx = 1usize;
						if i + 1 < tokens.len() {
							if let Token::Subscript(n) = &tokens[i + 1] {
								idx = *n as usize;
								i += 1;
							}
						}
						parts.push(MoleculePart::Element { element: elem, index: idx });
						i += 1;
					}
					Token::LeftParenthesis => {
						let (inner_parts, next_i) = parse_parts(tokens, i + 1, resolve_element)?;
						i = next_i;
						if i >= tokens.len() {
							return Err(ParserError::MissingParenthesis);
						}
						match &tokens[i] {
							Token::RightParenthesis => {
								i += 1;
								let mut gidx = 1usize;
								if i < tokens.len() {
									if let Token::Subscript(n) = &tokens[i] {
										gidx = *n as usize;
										i += 1;
									}
								}
								parts.push(MoleculePart::Group { parts: inner_parts, index: gidx });
							}
							_ => return Err(ParserError::MissingParenthesis),
						}
					}
					Token::RightParenthesis => {
						return Ok((parts, i));
					}
					Token::Subscript(_) => return Err(ParserError::UnexpectedSubscript),
					Token::Coefficient(_) | Token::Plus | Token::Arrow(_) => break,
					_ => { i += 1; }
				}
			}
			Ok((parts, i))
		}

		if i < tokens.len() {
			if let Token::Coefficient(c) = &tokens[i] {
				coefficient = *c;
				i += 1;
			}
		}

		let (parts, _next) = parse_parts(&tokens, i, &resolve_element)?;
		Ok(Self { coefficient, parts })
	}
}

/// Count atoms in the molecule
pub(crate) fn count_atom_occurrences(mol: &Molecule) -> BTreeMap<String, usize> {
	let mut out: BTreeMap<String, usize> = BTreeMap::new();
	fn add_counts(map: &mut BTreeMap<String, usize>, part: &MoleculePart, multiplier: usize) {
		match part {
			MoleculePart::Element { element, index } => {
				let e = element.symbol.clone();
				*map.entry(e).or_insert(0) += index * multiplier;
			}
			MoleculePart::Group { parts, index } => {
				let m = multiplier * (*index);
				for p in parts {
					add_counts(map, p, m);
				}
			}
		}
	}
	for p in &mol.parts {
		add_counts(&mut out, p, 1);
	}
	out
}

impl Display for MoleculePart {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MoleculePart::Element { element, index } => {
				write!(f, "{}", element.symbol)?;
				if *index != 1 { write!(f, "{}", index)?; }
				Ok(())
			}
			MoleculePart::Group { parts, index } => {
				write!(f, "(")?;
				for p in parts { write!(f, "{}", p)?; }
				write!(f, ")")?;
				if *index != 1 { write!(f, "{}", index)?; }
				Ok(())
			}
		}
	}
}

impl Display for Molecule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.coefficient != 1 { write!(f, "{}", self.coefficient)?; }
		for p in &self.parts { write!(f, "{}", p)?; }
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;
	use crate::parser::molecule::{count_atom_occurrences, Molecule};

	#[test]
	fn molecule() {
		let string = "2H2O";
		let lexer = crate::parser::token::Lexer::new(string);
		let expr = lexer.tokenize().unwrap();
		let molecule = Molecule::from_tokens(expr).unwrap();
		assert_eq!(molecule.coefficient, 2);
	}

	#[test]
	fn count_atoms() {
		let string = "C8H9AsBiNO6";
		let lexer = crate::parser::token::Lexer::new(string);
		let expr = lexer.tokenize().unwrap();
		let molecule = Molecule::from_tokens(expr).unwrap();

		let mut sample: BTreeMap<String, usize> = BTreeMap::new();
		sample.insert(String::from("As"), 1);
		sample.insert(String::from("Bi"), 1);
		sample.insert(String::from("C"), 8);
		sample.insert(String::from("H"), 9);
		sample.insert(String::from("N"), 1);
		sample.insert(String::from("O"), 6);

		assert_eq!(count_atom_occurrences(&molecule), sample);
	}
}