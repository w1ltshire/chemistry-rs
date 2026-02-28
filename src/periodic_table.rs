use serde_json::Value;
use crate::element::Element;

#[derive(Debug)]
/// The periodic table
pub struct PeriodicTable {
	/// All the chemical elements
	pub elements: Vec<Element>,
}

impl PeriodicTable {
	/// Creates a new instance of [`PeriodicTable`]
	pub fn new() -> Self {
		let mut value: Value = serde_json::from_str(include_str!("../periodic-table/PeriodicTableJSON.json")).unwrap();
		let elements = serde_json::from_value(value["elements"].take()).unwrap();
		PeriodicTable { elements }
	}
}