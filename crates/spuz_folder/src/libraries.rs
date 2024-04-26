use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use tokio::fs::try_exists;

use crate::Result;

#[derive(Debug)]
pub struct Libraries {
	path: Arc<Path>,
}

impl Libraries {
	pub(crate) fn from_folder_root(root: &Path) -> Self {
		let path = root.join("libraries").into();
		Self { path }
	}

	pub fn get(&self, name: &str) -> Lib {
		Lib::new(self.path.clone(), name)
	}
}

type LazyPath = std::cell::OnceCell<Arc<Path>>;

#[derive(Debug)]
pub struct Lib {
	pub name: String,
	libraries_dir: Arc<Path>,
	path: LazyPath,
}

impl Lib {
	pub(crate) fn new(libraries_dir: Arc<Path>, name: &str) -> Self {
		Self { name: name.into(), libraries_dir, path: LazyPath::new() }
	}

	pub fn path(&self) -> &Arc<Path> {
		self.path.get_or_init(|| {
			let parts: Vec<&str> = self.name.split(':').collect();
			let mut path_buf = PathBuf::new();

			let groups = parts[0].split('.');
			let name = parts[1];
			let version = parts[2];

			for group in groups {
				path_buf.push(group);
			}

			path_buf.push(name);
			path_buf.push(version);
			path_buf.push(format!("{name}-{version}.jar"));

			self.libraries_dir.join(path_buf).into()
		})
	}

	pub async fn exists(&self) -> Result<bool> {
		try_exists(self.path()).await.map_err(Into::into)
	}
}
