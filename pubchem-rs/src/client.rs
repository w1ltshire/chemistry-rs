use std::collections::HashMap;
use crate::compound::{Compound, CompoundResponse};
use crate::errors::{ApiError, ApiResult, FaultRoot};
use crate::fast_formula::IdentifierListResponse;

/// An asynchronous `Client` to make Requests with.
pub struct Client {
	#[cfg(feature = "async")]
	http: reqwest::Client,
	#[cfg(feature = "sync")]
	http: reqwest::blocking::Client,
	cid_cache: HashMap<String, isize> // formula-cid
}

/// Base URL for the Pubchem REST API
pub(crate) const BASE_URL: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound";

impl Client {
	/// Create a new instance of [`Client`].
	///
	/// Will use a [`reqwest::Client`] if `async` feature is enabled, if `sync` - [`reqwest::blocking::Client`]
	pub fn new() -> Self {
		Self {
			#[cfg(feature = "async")]
			http: reqwest::Client::new(),
			#[cfg(feature = "sync")]
			http: reqwest::blocking::Client::new(),
			cid_cache: HashMap::new()
		}
	}

	/// Retrieve a compound ID from Pubchem API by its chemical formula
	pub async fn cid_by_formula(&mut self, formula: &str) -> ApiResult<isize> {
		if let Some(&cid) = self.cid_cache.get(formula) {
			return Ok(cid);
		}
		let url = format!("{BASE_URL}/fastformula/{formula}/cids/JSON?AllowOtherElements=false&MaxRecords=1");
		let cid = self.get_json::<IdentifierListResponse>(&*url).await?.identifier_list.compound_ids[0];
		self.cid_cache.insert(formula.to_string(), cid);
		Ok(cid)
	}

	/// Retrieve a compound from Pubchem API by its chemical formula
	pub async fn compound_by_formula(&mut self, formula: &str) -> ApiResult<Compound> {
		let cid = self.cid_by_formula(formula).await?;
		self.compound_by_cid(cid).await
	}

	/// Retrieve a compound from Pubchem API by its compound ID
	pub async fn compound_by_cid(&self, cid: isize) -> ApiResult<Compound> {
		let url = format!("{BASE_URL}/cid/{cid}/json");
		Ok(self.get_json::<CompoundResponse>(&*url).await?.pc_compounds[0].clone())
	}

	async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> ApiResult<T> {
		tracing::debug!(url = %url, "sending request");
		let resp = self.http.get(url).send().await?;
		let status = resp.status();
		let bytes = resp.bytes().await?;
		if status.is_success() {
			Ok(serde_json::from_slice(&bytes)?)
		} else {
			let fault: FaultRoot = serde_json::from_slice(&bytes)?;
			Err(ApiError::ServerError(fault.fault))
		}
	}
}

#[cfg(test)]
mod tests {
	use tracing_test::traced_test;
	use crate::client::Client;

	#[traced_test]
	#[tokio::test]
	async fn cid_by_formula() {
		let mut client = Client::new();
		assert_eq!(client.cid_by_formula("H2O").await.unwrap(), 962);
	}

	#[traced_test]
	#[tokio::test]
	async fn compound_by_cid() {
		let client = Client::new();
		client.compound_by_cid(962).await.unwrap();
	}

	#[traced_test]
	#[tokio::test]
	async fn compound_by_formula() {
		let mut client = Client::new();
		let compound = client.compound_by_formula("MgO").await.unwrap();
		tracing::debug!("{:?}, {:?}, {:?}", compound.mass(), compound.name(), compound.smiles());
	}
}