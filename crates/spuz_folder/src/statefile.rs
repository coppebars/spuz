use std::{
	collections::HashMap,
	io::ErrorKind,
	ops::{Deref, DerefMut},
	path::Path,
	str::FromStr,
	sync::Arc,
};

use serde::{Deserialize, Serialize};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
	sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock},
};
use toml::{from_str, to_string};

use crate::{Error, Result};

pub const FILENAME: &str = "spzst.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
	pub name: Box<str>,
	pub id: Box<str>,
	pub version: Box<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
	pub integrity: HashMap<Box<Path>, Box<str>>,
	pub versions: Vec<Version>,
}

#[allow(clippy::derivable_impls)]
impl Default for State {
	fn default() -> Self {
		Self { integrity: HashMap::new(), versions: Vec::new() }
	}
}

impl FromStr for State {
	type Err = Error;

	fn from_str(str: &str) -> std::result::Result<Self, Self::Err> {
		from_str(str).map_err(Into::into)
	}
}

impl State {
	pub fn to_string(&self) -> Result<String> {
		to_string(self).map_err(Into::into)
	}

	pub async fn load(path: &Path) -> Result<Self> {
		let mut file = match File::open(path).await {
			Ok(it) => it,
			Err(err) if err.kind() == ErrorKind::NotFound => {
				let mut file = File::create(path).await?;
				let default_content = Self::default().to_string()?;
				file.write_all(default_content.as_bytes()).await?;
				File::open(path).await?
			}
			err => err?,
		};
		let mut content = String::new();
		file.read_to_string(&mut content).await?;
		Self::from_str(&content)
	}

	pub async fn write(&self, path: &Path) -> Result<()> {
		let mut file = File::create(path).await?;
		let content = self.to_string()?;
		file.write_all(content.as_bytes()).await?;
		Ok(())
	}
}

#[derive(Debug)]
pub struct StateRead {
	state: OwnedRwLockReadGuard<State>,
}

impl StateRead {
	async fn new(rw: Arc<RwLock<State>>) -> Self {
		Self { state: rw.read_owned().await }
	}
}

impl Deref for StateRead {
	type Target = OwnedRwLockReadGuard<State>;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

#[derive(Debug)]
pub struct StateWrite {
	state: OwnedRwLockWriteGuard<State>,
	path: Arc<Path>,
}

impl StateWrite {
	async fn new(rw: Arc<RwLock<State>>, path: Arc<Path>) -> Self {
		Self { state: rw.write_owned().await, path }
	}

	pub async fn save(&self) -> Result<()> {
		self.write(&self.path).await
	}
}

impl Deref for StateWrite {
	type Target = OwnedRwLockWriteGuard<State>;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

impl DerefMut for StateWrite {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.state
	}
}

#[derive(Debug, Clone)]
pub struct Statefile {
	state: Arc<RwLock<State>>,
	path: Arc<Path>,
}

impl Statefile {
	pub async fn load(path: &Path) -> Result<Self> {
		let raw_state = State::load(path).await?;
		let state = Arc::new(RwLock::new(raw_state));
		Ok(Self { state, path: path.into() })
	}

	pub async fn read(&self) -> StateRead {
		StateRead::new(Arc::clone(&self.state)).await
	}

	pub async fn write(&self) -> StateWrite {
		StateWrite::new(Arc::clone(&self.state), Arc::clone(&self.path)).await
	}
}
