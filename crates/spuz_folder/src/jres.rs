use std::{path::Path, sync::Arc};

use cfg_if::cfg_if;
use tokio::fs::try_exists;

use crate::Result;

#[derive(Debug)]
pub struct Jres {
	path: Arc<Path>,
}

impl Jres {
	pub(crate) fn from_folder_root(path: &Path) -> Self {
		let path = path.join("jres").into();
		Self { path }
	}

	pub fn get(&self, component: &str) -> JavaRuntime {
		JavaRuntime::new(self.path.clone(), component)
	}
}

type LazyPath = std::cell::OnceCell<Arc<Path>>;

#[derive(Debug)]
pub struct JavaRuntime {
	pub component: String,
	jres_dir: Arc<Path>,
	bin: LazyPath,
}

impl JavaRuntime {
	pub(crate) fn new(jres_dir: Arc<Path>, component: &str) -> Self {
		Self { jres_dir, component: component.into(), bin: LazyPath::new() }
	}

	pub fn bin(&self) -> &Arc<Path> {
		self.bin.get_or_init(|| {
			let component_dir = self.jres_dir.join(&self.component);

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

	pub async fn exists(&self) -> Result<bool> {
		try_exists(self.bin()).await.map_err(Into::into)
	}
}
