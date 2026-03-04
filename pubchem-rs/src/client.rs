/// An asynchronous `Client` to make Requests with.
pub struct Client {
	#[cfg(feature = "async")]
	http: reqwest::Client,
	#[cfg(feature = "sync")]
	http: reqwest::blocking::Client
}

impl Client {
	/// Create a new instance of [`Client`].
	///
	/// Will use a [`reqwest::Client`] if `async` feature is enabled, if `sync` feature - [`reqwest::blocking::Client`]
	pub fn new() -> Self {
		Self {
			#[cfg(feature = "async")]
			http: reqwest::Client::new(),
			#[cfg(feature = "sync")]
			http: reqwest::blocking::Client::new()
		}
	}
}
