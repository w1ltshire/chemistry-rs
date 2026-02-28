use crate::parser::token::Token;
use crate::parser::token::Token::{Arrow, Coefficient, Element, Plus, Subscript};
use crate::periodic_table::PeriodicTable;

#[test]
fn read_periodic_table() {
	let periodic_table = PeriodicTable::new();
	assert_eq!(periodic_table.elements.len(), 119);
	assert_eq!(periodic_table.elements[49].name, "Tin");
}

#[test]
fn tokenize() {
	let string = "2H2 + O2 -> H2O";
	let lexer = crate::parser::token::Lexer::new(string);
	let tokens = lexer.tokenize().unwrap();
	println!("{:?}", tokens);
}

#[test]
fn tokenize_coefficient() {
	let string = "2H2 + O2 -> 2H2O";
	let lexer = crate::parser::token::Lexer::new(string);
	let tokens = lexer.tokenize().unwrap();
	let sample = vec![
		Coefficient(2),
		Element(String::from("H")),
		Subscript(2),
		Plus,
		Element(String::from("O")),
		Subscript(2),
		Arrow(String::from("->")),
		Coefficient(2),
		Element(String::from("H")),
		Subscript(2),
		Element(String::from("O"))
	];
	assert_eq!(tokens, sample);
}

#[test]
#[should_panic]
fn tokenize_invalid_syntax() {
	let string = "H2 + O/2 --> H2O";
	let lexer = crate::parser::token::Lexer::new(string);
	lexer.tokenize().unwrap();
}