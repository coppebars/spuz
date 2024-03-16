use std::io::Error as IoError;

use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	Reqwest(#[from] ReqwestError),
}
