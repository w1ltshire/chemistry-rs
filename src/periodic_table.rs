use crate::element::Element;
use lazy_static::lazy_static;
use serde_json::Value;

#[derive(Debug)]
/// The periodic table.
///
/// When accessing an element in the `elements` vector by its atomic number note that arrays
/// start from 0, so if your element has an atomic number of 50 you need to access 49th element
/// in the array.
pub struct PeriodicTable {
    /// All the chemical elements
    pub elements: Vec<Element>,
}

lazy_static! {
    /// Global periodic table instance
    pub static ref PERIODIC_TABLE: PeriodicTable = PeriodicTable::new();
}

impl PeriodicTable {
    /// Creates a new instance of [`PeriodicTable`]
    pub fn new() -> Self {
        let mut value: Value =
            serde_json::from_str(include_str!("../periodic-table/PeriodicTableJSON.json")).unwrap();
        let elements = serde_json::from_value(value["elements"].take()).unwrap();
        PeriodicTable { elements }
    }
}
