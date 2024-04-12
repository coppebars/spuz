use std::io::Error as IoError;

use reqwest::Error as ReqwestError;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	Reqwest(#[from] ReqwestError),
}

impl Error {
	pub fn is_critical(&self) -> bool {
		match self {
			Error::Io(_) => true,
			Error::Reqwest(err)
				if err.is_status() || err.is_decode() || err.is_body() || err.is_redirect() || err.is_request() =>
			{
				true
			}
			Error::Reqwest(_) => false,
		}
	}
}
