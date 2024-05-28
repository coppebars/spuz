use std::fmt::Debug;

use thiserror::Error;

use crate::{
	ext::{FsExtLoadError, FsExtSaveError},
	json_resource::{JsonResourceParseError, JsonResourceSaveError},
	Client, FetchError,
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("FetchError: {0}")]
	Fetch(#[source] Box<dyn std::error::Error>),
	#[error(transparent)]
	JsonResourceSave(#[from] JsonResourceSaveError),
	#[error(transparent)]
	JsonResourceParse(#[from] JsonResourceParseError),
	#[error(transparent)]
	FsExtSave(#[from] FsExtSaveError),
	#[error(transparent)]
	FsExtLoda(#[from] FsExtLoadError),
}

impl Error {
	pub fn from_fetch<C: Client + Debug + 'static>(err: FetchError<C>) -> Self {
		Self::Fetch(Box::new(err))
	}
}
