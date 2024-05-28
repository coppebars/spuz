use std::{io, path::Path};

use async_trait::async_trait;
use spuz_piston::{AssetIndex, Manifest, RuntimeManifest};
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Error)]
#[error("{0}")]
pub enum FsExtLoadError {
	ReadFile(
		#[from]
		#[source]
		io::Error,
	),
	Deserialize(
		#[from]
		#[source]
		serde_json::Error,
	),
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum FsExtSaveError {
	CreateFile(#[source] io::Error),
	Copy(#[source] io::Error),
}

/// Allows you to [save](Self::save) or [load](Self::load) documents from the
/// file system
#[async_trait]
pub trait FsExt: Sized {
	/// Saves the document locally
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// # use pollster::FutureExt;
	/// use spuz_piston::Manifest;
	/// use spuz_get::FsExt;
	///
	/// # async move {
	/// let manifest = Manifest::load("./1.20.6.json").await?;
	/// manifest.save("./1.20.6-new.json").await?;
	/// # Result::<(), Box<dyn Error>>::Ok(())
	/// # }.block_on();
	/// ```
	async fn save(&self, path: impl AsRef<Path> + Send) -> Result<(), FsExtSaveError>;
	/// Loads the document from the fs
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// # use pollster::FutureExt;
	/// use spuz_piston::Manifest;
	/// use spuz_get::FsExt;
	///
	/// # async move {
	/// let manifest = Manifest::load("./1.20.6.json").await?;
	/// manifest.save("./1.20.6-new.json").await?;
	/// # Result::<(), Box<dyn Error>>::Ok(())
	/// # }.block_on();
	/// ```
	async fn load(path: impl AsRef<Path> + Send) -> Result<Self, FsExtLoadError>;
}

macro_rules! save_ext {
	($what:ident) => {
		#[async_trait]
		impl FsExt for $what {
			async fn save(&self, path: impl AsRef<Path> + Send) -> Result<(), FsExtSaveError> {
				let mut file = fs::File::create(path).await.map_err(FsExtSaveError::CreateFile)?;
				file.write_all(self.to_string().as_bytes()).await.map_err(FsExtSaveError::Copy)?;
				Ok(())
			}

			async fn load(path: impl AsRef<Path> + Send) -> Result<Self, FsExtLoadError> {
				let content = fs::read_to_string(path).await?;
				content.parse().map_err(Into::into)
			}
		}
	};
}

save_ext!(Manifest);
save_ext!(AssetIndex);
save_ext!(RuntimeManifest);
