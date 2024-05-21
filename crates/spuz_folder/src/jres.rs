use std::{path::Path, sync::Arc};

use cfg_if::cfg_if;
use tokio::fs::try_exists;

use crate::Result;

#[derive(Debug)]
pub struct JavaRuntimes {
	path: Arc<Path>,
}

impl JavaRuntimes {
	pub(crate) fn from_folder_root(path: &Path) -> Self {
		let path = path.join("jres").into();
		Self { path }
	}

	pub fn component(&self, component: impl Into<Arc<str>>) -> JavaRuntime {
		JavaRuntime::new(self.path.clone(), component.into())
	}
}

type LazyPath = std::cell::OnceCell<Arc<Path>>;

#[derive(Debug)]
pub struct JavaRuntime {
	pub component: Arc<str>,
	pub parent_dir: Arc<Path>,
	pub dir: LazyPath,
	bin: LazyPath,
}

impl JavaRuntime {
	pub(crate) fn new(parent_dir: Arc<Path>, component: Arc<str>) -> Self {
		Self { parent_dir, component, bin: LazyPath::new(), dir: LazyPath::new() }
	}

	pub fn dir(&self) -> &Arc<Path> {
		self.dir.get_or_init(|| self.parent_dir.join(&*self.component).into())
	}

	pub fn bin(&self) -> &Arc<Path> {
		self.bin.get_or_init(|| {
			let component_dir = self.dir();

			cfg_if! {
				if #[cfg(target_os = "windows")] {
					let ext = ".exe";
				} else {
					let ext = "";
				}
			}

			component_dir.join("bin").join(format!("java{ext}")).into()
		})
	}

	pub async fn bin_exists(&self) -> Result<bool> {
		try_exists(self.bin()).await.map_err(Into::into)
	}
}
