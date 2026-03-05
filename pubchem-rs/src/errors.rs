use serde::{Deserialize, Serialize};
use thiserror::Error;

/// [`Result`] wrapper type for [`ApiError`] for convenience
pub type ApiResult<T> = Result<T, ApiError>;

/// Possible API errors
#[derive(Error, Debug)]
pub enum ApiError {
	/// An HTTP request error
	#[error(transparent)]
	HttpError(#[from] reqwest::Error),
	#[error("API has returned an error: `{0:?}`")]
	ServerError(Fault),
	#[error(transparent)]
	JsonError(#[from] serde_json::error::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct FaultRoot {
	#[serde(rename = "Fault")]
	pub fault: Fault,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fault {
	pub code: String,
	pub message: String
}