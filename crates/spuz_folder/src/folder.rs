use std::{path::Path, sync::Arc};

use crate::{statefile, CommonDirs, Result, Statefile};

#[derive(Debug, Clone)]
pub struct Folder {
	pub root: Arc<Path>,
	pub statefile: Statefile,
	pub common_dirs: Arc<CommonDirs>,
}

impl Folder {
	pub async fn settle(root: impl AsRef<Path>) -> Result<Self> {
		let root: Arc<Path> = Arc::from(root.as_ref());
		let statefile = Statefile::load(&root.join(statefile::FILENAME)).await?;
		let common_dirs = Arc::new(CommonDirs::for_root(&root).await?);

		Ok(Self {
			root,
			statefile,
			common_dirs,
		})
	}
}
