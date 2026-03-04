use std::collections::HashMap;
use std::fmt::Display;
use fraction::Integer;
use num_rational::Rational64;
use crate::parser::error::{ParserError, ParserResult};
use crate::molecule::Molecule;
use crate::parser::token::{Lexer, Token};

/// Structure representing a chemical reaction with reactants and products
#[derive(Debug)]
pub struct Reaction {
	/// Array of reactants
	pub reactants: Vec<Molecule>,
	/// Array of reaction products
	pub products: Vec<Molecule>,
}

fn iter_split_on_plus(tokens: &[Token]) -> Vec<&[Token]> {
	tokens.split(|t| matches!(t, Token::Plus)).collect::<Vec<&[Token]>>()
}

impl Reaction {
	/// Create a [`Reaction`] instance from an array of tokens
	///
	/// # Example
	/// ```rust
	/// use chem_equations::reaction::Reaction;
	/// use chem_equations::parser::token::{Token, Lexer};
	///
	/// let input = "H2 + Cl2 => 2HCl";
	/// let tokens = Lexer::new(input).tokenize().unwrap();
	/// let reaction = Reaction::from_tokens(tokens).unwrap();
	/// ```
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

	/// Create a [`Reaction`] instance from a string
	///
	/// # Example
	/// ```rust
	/// use chem_equations::reaction::Reaction;
	///
	/// let input = "H2 + Cl2 => 2HCl";
	/// let reaction = Reaction::from_string(input).unwrap();
	/// ```
	pub fn from_string(s: &str) -> ParserResult<Reaction> {
		let expr = Lexer::new(s).tokenize()?;
		Reaction::from_tokens(expr)
	}

	pub fn balance(&mut self) {
		let all_mols: Vec<&Molecule> = self.reactants.iter().chain(self.products.iter()).collect();

		let mut elem_index: HashMap<String, usize> = HashMap::new();
		let mut elements: Vec<String> = Vec::new();
		let mut mol_counts: Vec<HashMap<String, i64>> = Vec::with_capacity(all_mols.len());

		for mol in &all_mols {
			let counts = mol.collect_element_counts();
			for k in counts.keys() {
				if !elem_index.contains_key(k) {
					elem_index.insert(k.clone(), elements.len());
					elements.push(k.clone());
				}
			}
			mol_counts.push(counts);
		}

		let m = elements.len();
		let n = all_mols.len();
		if m == 0 || n == 0 { return; }

		let mut a: Vec<Vec<Rational64>> = vec![vec![Rational64::from_integer(0); n]; m];
		for (col, counts) in mol_counts.iter().enumerate() {
			let sign = if col < self.reactants.len() { Rational64::from_integer(1) } else { Rational64::from_integer(-1) };
			for (elem, &cnt) in counts.iter() {
				let row = elem_index[elem];
				a[row][col] = a[row][col] + sign * Rational64::from_integer(cnt);
			}
		}

		let mut mat = a;
		let mut row = 0usize;
		let mut pivot_cols: Vec<Option<usize>> = vec![None; m];

		for col in 0..n {
			if row >= m { break; }
			let mut sel = None;
			for r in row..m {
				if mat[r][col] != Rational64::from_integer(0) {
					sel = Some(r);
					break;
				}
			}
			if let Some(rsel) = sel {
				mat.swap(row, rsel);
				let pivot = mat[row][col];
				for c in col..n {
					mat[row][c] = mat[row][c] / pivot;
				}
				for r in 0..m {
					if r != row {
						let factor = mat[r][col];
						if factor != Rational64::from_integer(0) {
							for c in col..n {
								mat[r][c] = mat[r][c] - factor * mat[row][c];
							}
						}
					}
				}
				pivot_cols[row] = Some(col);
				row += 1;
			}
		}

		let mut is_pivot_col = vec![false; n];
		for &pc in &pivot_cols {
			if let Some(c) = pc { is_pivot_col[c] = true; }
		}
		let free_cols: Vec<usize> = (0..n).filter(|&c| !is_pivot_col[c]).collect();
		if free_cols.is_empty() {
			return;
		}

		let mut solution: Vec<Rational64> = vec![Rational64::from_integer(0); n];
		let free_choice = free_cols[0];
		solution[free_choice] = Rational64::from_integer(1);

		for (r, pc_opt) in pivot_cols.iter().enumerate() {
			if let Some(pc) = pc_opt {
				let mut val = Rational64::from_integer(0);
				for &fc in &free_cols {
					let coeff = mat[r][fc];
					if coeff != Rational64::from_integer(0) {
						val = val + coeff * solution[fc];
					}
				}
				solution[*pc] = -val;
			}
		}

		let mut reactant_sum = Rational64::from_integer(0);
		for i in 0..self.reactants.len() {
			reactant_sum = reactant_sum + solution[i];
		}
		if reactant_sum < Rational64::from_integer(0) {
			for s in solution.iter_mut() { *s = -*s; }
		}

		let mut lcm_den: i64 = 1;
		for r in &solution {
			let d = r.denom().abs();
			lcm_den = num_integer::lcm(lcm_den, d);
		}
		if lcm_den == 0 { lcm_den = 1; }
		let mut ints: Vec<i64> = solution.iter().map(|r| (r * Rational64::from_integer(lcm_den)).to_integer()).collect();

		for v in ints.iter_mut() {
			if *v < 0 { *v = -*v; }
			if *v == 0 { *v = 1; }
		}

		let mut g = ints[0].abs();
		for &c in ints.iter().skip(1) { g = g.gcd(&c.abs()); }
		if g == 0 { g = 1; }
		for v in ints.iter_mut() { *v /= g; }

		for (i, mol) in self.reactants.iter_mut().enumerate() {
			mol.coefficient = ints[i] as isize;
		}
		for (j, mol) in self.products.iter_mut().enumerate() {
			mol.coefficient = ints[self.reactants.len() + j] as isize;
		}
	}
}

impl Display for Reaction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, molecule) in self.reactants.iter().enumerate() {
			if i > 0 { write!(f, " + ")?; }
			write!(f, "{}", molecule)?;
		}

		write!(f, " → ")?;

		for (i, molecule) in self.products.iter().enumerate() {
			if i > 0 { write!(f, " + ")?; }
			write!(f, "{}", molecule)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn reaction() {
		let string = "Al(OH)3 + H2SO4 -> Al2(SO4)3 + H2O";
		let lexer = crate::parser::token::Lexer::new(string);
		let expr = lexer.tokenize().unwrap();
		let reaction = crate::reaction::Reaction::from_tokens(expr).unwrap();
		println!("{}", reaction);
	}

	#[test]
	fn reaction_from_string() {
		let expr = "H2 + Cl2 -> 2HCl";
		let reaction = crate::reaction::Reaction::from_string(expr).unwrap();
		println!("{}", reaction);
	}

	#[test]
	#[should_panic]
	fn reaction_invalid_syntax() {
		let string = "2H2 + O/2 - 2H2O";
		let lexer = crate::parser::token::Lexer::new(string);
		let expr = lexer.tokenize().unwrap();
		crate::reaction::Reaction::from_tokens(expr).unwrap();
	}

	#[test]
	fn balance() {
		let string = "Al(OH)3 + H2SO4 -> Al2(SO4)3 + H2O";
		let lexer = crate::parser::token::Lexer::new(string);
		let expr = lexer.tokenize().unwrap();
		let mut reaction = crate::reaction::Reaction::from_tokens(expr).unwrap();
		reaction.balance();
		println!("{}", reaction);
	}
}
