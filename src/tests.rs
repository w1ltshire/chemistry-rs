use crate::periodic_table::PeriodicTable;

#[test]
fn read_periodic_table() {
	let periodic_table = PeriodicTable::new();
	assert_eq!(periodic_table.elements.len(), 119);
	assert_eq!(periodic_table.elements[49].name, "Tin");
}