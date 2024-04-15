use std::{path::Path, sync::Arc};

use tokio::fs::{canonicalize, create_dir_all};
use tracing::info;

use crate::Result;

#[derive(Debug, Clone)]
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
}
