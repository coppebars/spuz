use std::{path::Path, sync::Arc};

pub use err::{Error, Result};
use instances::Instances;
use tokio::fs::{canonicalize, create_dir_all};
use tracing::info;
use versions::Versions;

mod err;
mod instances;
mod versions;

#[derive(Debug)]
pub struct Folder {
	pub root: Arc<Path>,
}

impl Folder {
	pub async fn settle(root: impl AsRef<Path>) -> Result<Arc<Self>> {
		let root = canonicalize(root.as_ref()).await?.into();
		create_dir_all(&root).await?;

		info!("Spuz folder settled into {root:?}");

		Ok(Self { root }.into())
	}

	pub fn versions(&self) -> Versions {
		Versions::from_folder_root(&self.root)
	}

	pub fn instances(&self) -> Instances {
		Instances::from_folder_root(&self.root)
	}
}
