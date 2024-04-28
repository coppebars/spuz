use std::io::Error as IoError;

use hex::FromHexError;
use spuz_piston::Error as PistonError;
use thiserror::Error;
use tokio::task::JoinError;
use toml::{de::Error as TomlDe, ser::Error as TomlSer};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] IoError),

	#[error(transparent)]
	TomlDe(#[from] TomlDe),

	#[error(transparent)]
	TomlSer(#[from] TomlSer),

	#[error(transparent)]
	FromHex(#[from] FromHexError),

	#[error(transparent)]
	Join(#[from] JoinError),

	#[error(transparent)]
	Piston(#[from] PistonError),
}
