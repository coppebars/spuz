use std::io::Error as IoError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] IoError),
}
