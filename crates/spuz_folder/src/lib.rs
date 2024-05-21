mod err;
mod jres;

use std::{path::Path, sync::Arc};

use tokio::fs::{canonicalize, create_dir_all};
use tracing::info;

pub use crate::{
	err::{Error, Result},
	jres::JavaRuntimes,
};

#[derive(Debug)]
pub struct Folder {
	pub root: Arc<Path>,
	pub java_runtimes: JavaRuntimes,
}

impl Folder {
	pub async fn settle(root: impl AsRef<Path>) -> Result<Arc<Self>> {
		let root = Arc::from(canonicalize(root.as_ref()).await?);
		create_dir_all(&root).await?;

		info!("Spuz folder settled into {root:?}");

		let java_runtimes = JavaRuntimes::from_folder_root(&root);

		Ok(Self { root, java_runtimes }.into())
	}
}
