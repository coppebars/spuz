use std::io::Error as IoError;

use thiserror::Error;

use spuz_piston::Error as PistonError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	Piston(#[from] PistonError),
}
