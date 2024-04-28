use serde_json::Error as JsonError;
use thiserror::Error;
use url::ParseError as UrlParseError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	UrlParse(#[from] UrlParseError),

	#[error(transparent)]
	Json(#[from] JsonError),
}
