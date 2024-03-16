use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SpuzFsp {
	root: PathBuf,
}

impl SpuzFsp {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}

	/// `/`
	pub fn root(&self) -> &Path {
		&self.root
	}

	/// `/versions`
	pub fn versions(&self) -> PathBuf {
		self.root.join("versions")
	}

	/// `/versions/{version}`
	pub fn version(&self, version: impl AsRef<str>) -> PathBuf {
		self.versions().join(version.as_ref())
	}

	/// `/libs`
	pub fn libs(&self) -> PathBuf {
		self.root.join("libs")
	}

	/// `/assets`
	pub fn assets(&self) -> PathBuf {
		self.root.join("assets")
	}

	/// `/assets/indexes`
	pub fn asset_indexes(&self) -> PathBuf {
		self.assets().join("indexes")
	}

	/// `/assets/objects`
	pub fn asset_objects(&self) -> PathBuf {
		self.assets().join("objects")
	}

	/// `/assets/objects/{hash[..2]}`
	pub fn asset_object(&self, hash: impl AsRef<str>) -> PathBuf {
		self.asset_objects().join(&hash.as_ref()[..2])
	}

	/// `/instances`
	pub fn instances(&self) -> PathBuf {
		self.root.join("instances")
	}

	/// `/instances/{name}`
	pub fn instance(&self, name: impl AsRef<str>) -> PathBuf {
		self.instances().join(name.as_ref())
	}
}
