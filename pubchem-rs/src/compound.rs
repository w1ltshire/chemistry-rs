use serde::{Deserialize, Deserializer, Serialize};

/// API response on compound endpoint
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CompoundResponse {
	#[serde(rename = "PC_Compounds")]
	pub pc_compounds: Vec<Compound>,
}

/// Structure representing a compound
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compound {
	#[serde(rename = "props")]
	props: Vec<Prop>,
}

/// Compound property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prop {
	/// Uniform resource name
	pub urn: Urn,
	/// Property name
	pub value: Value,
}

/// Uniform resource name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Urn {
	pub label: Option<String>,
	pub name: Option<String>,
	pub datatype: Option<u64>,
	pub implementation: Option<String>,
	pub version: Option<String>,
	pub software: Option<String>,
	pub source: Option<String>,
	pub release: Option<String>,
	pub parameters: Option<String>,
}

/// Possible property values
#[derive(Debug, Clone ,Serialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
	IVal { ival: i64 },
	FVal { fval: f64 },
	SVal { sval: String },
	Binary { binary: String },
	Empty,
}

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct Helper {
			ival: Option<i64>,
			fval: Option<f64>,
			sval: Option<String>,
			binary: Option<String>,
		}

		let helper = Helper::deserialize(deserializer)?;
		if let Some(v) = helper.ival {
			return Ok(Value::IVal { ival: v });
		}
		if let Some(v) = helper.fval {
			return Ok(Value::FVal { fval: v });
		}
		if let Some(v) = helper.sval {
			return Ok(Value::SVal { sval: v });
		}
		if let Some(v) = helper.binary {
			return Ok(Value::Binary { binary: v });
		}
		Ok(Value::Empty)
	}
}