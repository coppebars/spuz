use std::{
	fmt::{Debug, Formatter},
	marker::PhantomData,
	path::Path,
};

use async_compat::CompatExt;
use futures_lite::{io, AsyncRead};
use futures_util::AsyncReadExt;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::fs::File;

/// You can get `JsonResource` from some api calls. If you just need to get the
/// structure from the json response, use the [json](JsonResource::json) method.
/// Sometimes you may need to save the result to a file, then use the
/// [save](JsonResource::save) method to avoid unnecessary parsing.
///
/// It is recommended that you perform one of the actions immediately, as
/// prolonged inactivity may result in a timeout.
///
/// # Example
/// ```no_run
/// # use pollster::FutureExt;///
/// # async move {
/// let json_resource = todo!();
///
/// let result = json_resource.json().await?;
/// // Or
/// json_resource.save("./local.json").await?;
/// # Result::<(), spuz_get::Error>::Ok(())
/// # }.block_on()
/// ```
pub struct JsonResource<R, D> {
	pub(crate) stream: R,
	pub(crate) json: PhantomData<D>,
}

impl<R, D> Debug for JsonResource<R, D> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		"JsonResource { ... }".fmt(f)
	}
}

impl<R, D> JsonResource<R, D>
where
	R: AsyncRead + Unpin,
	D: DeserializeOwned,
{
	/// Reads the underlying stream to end and parses as [D]
	pub async fn json(&mut self) -> Result<D, JsonResourceParseError> {
		let mut content = String::new();
		self.stream.read_to_string(&mut content).await?;
		Ok(serde_json::from_str(&content)?)
	}

	/// Copies the underlying stream to the file
	pub async fn save(&mut self, path: impl AsRef<Path>) -> Result<(), JsonResourceSaveError> {
		let mut file = File::create(path).await.map_err(JsonResourceSaveError::CreateFile)?;
		io::copy(&mut self.stream, file.compat_mut()).await.map_err(JsonResourceSaveError::Copy)?;
		Ok(())
	}
}

#[derive(Debug, Error)]
#[error("JsonResource parse error: {0}")]
pub enum JsonResourceParseError {
	Read(
		#[source]
		#[from]
		io::Error,
	),
	Parse(
		#[source]
		#[from]
		serde_json::Error,
	),
}

#[derive(Debug, Error)]
#[error("JsonResource save error: {0}")]
pub enum JsonResourceSaveError {
	CreateFile(#[source] io::Error),
	Copy(#[source] io::Error),
}
