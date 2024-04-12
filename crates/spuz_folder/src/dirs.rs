use std::{path::Path, sync::Arc};

use tokio::fs;

use crate::Result;

macro_rules! create_dir_all_join {
  ($self:ident, $($path:ident),*$(,)?) => {
	  let _ = tokio::try_join!($(fs::create_dir_all(&$self.$path)),*)?;
  };
}

#[derive(Debug, Clone)]
pub struct CommonDirs {
	pub versions: Arc<Path>,
	pub libraries: Arc<Path>,
	pub assets: Arc<Path>,
	pub asset_indexes: Arc<Path>,
	pub asset_objects: Arc<Path>,
	pub instances: Arc<Path>,
}

impl CommonDirs {
	pub(crate) async fn for_root(root: &Path) -> Result<Self> {
		let this = Self {
			versions: root.join("versions").into(),
			libraries: root.join("libraries").into(),
			assets: root.join("assets").into(),
			asset_indexes: root.join("assets").join("indexes").into(),
			asset_objects: root.join("assets").join("objects").into(),
			instances: root.join("instances").into(),
		};

		this.init().await?;

		Ok(this)
	}

	async fn init(&self) -> Result<()> {
		create_dir_all_join!(
			self,
			versions,
			libraries,
			assets,
			asset_indexes,
			asset_objects,
			instances,
		);

		Ok(())
	}
}
