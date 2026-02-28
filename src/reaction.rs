use crate::parser::error::{ParserError, ParserResult};
use crate::parser::molecule::Molecule;
use crate::parser::token::{Lexer, Token};

#[derive(Debug)]
pub struct Reaction {
	pub reactants: Vec<Molecule>,
	pub products: Vec<Molecule>,
}

fn iter_split_on_plus(tokens: &[Token]) -> Vec<&[Token]> {
	tokens.split(|t| matches!(t, Token::Plus)).collect::<Vec<&[Token]>>()
}

impl Reaction {
	pub fn from_tokens(tokens: Vec<Token>) -> ParserResult<Reaction> {
		let pos = tokens
			.iter()
			.position(|t| matches!(t, Token::Arrow(_)))
			.ok_or_else(|| ParserError::MissingArrow)?;

		let reactant_tokens = &tokens[..pos];
		let product_tokens = if pos + 1 <= tokens.len() { &tokens[pos + 1..] } else { &[] };

		fn parse_groups(
			groups: Vec<&[Token]>,
		) -> ParserResult<Vec<Molecule>> {
			let mut out = Vec::with_capacity(groups.len());
			for (_, slice) in groups.into_iter().enumerate() {
				let tok_vec = slice.to_vec();
				match Molecule::from_tokens(tok_vec) {
					Ok(m) => out.push(m),
					Err(e) => {
						return Err(e)
					}
				}
			}
			Ok(out)
		}

		let reactant_groups = iter_split_on_plus(reactant_tokens);
		let product_groups = iter_split_on_plus(product_tokens);

		let reactants = parse_groups(reactant_groups)?;
		let products = parse_groups(product_groups)?;

		Ok(Self { reactants, products })
	}

	pub fn from_string(s: &str) -> ParserResult<Reaction> {
		let expr = Lexer::new(s).tokenize()?;
		Reaction::from_tokens(expr)
	}
}
