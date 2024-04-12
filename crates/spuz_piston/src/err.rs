use thiserror::Error;
use url::ParseError as UrlParseError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	UrlParse(#[from] UrlParseError),

	#[cfg(feature = "api")]
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
}
