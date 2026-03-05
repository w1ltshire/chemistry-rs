use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IdentifierListResponse {
	#[serde(rename = "IdentifierList")]
	pub identifier_list: IdentifierList,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IdentifierList {
	#[serde(rename = "CID")]
	pub compound_ids: Vec<isize>,
}
