use async_trait::async_trait;
use futures_lite::AsyncRead;
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

pub(crate) type BoxedAsyncRead = Box<dyn AsyncRead + Unpin + Send + Sync + 'static>;

#[async_trait]
pub trait Client {
	type Error: std::error::Error + 'static;

	/// Sends a GET request and deserializes the result as json
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// use spuz_get::Client;
	/// use spuz_piston::Manifest;
	/// # type Result<T> = std::result::Result<T, Box<dyn Error>>;
	///
	/// async fn get_manifest<C: Client>(client: &C) -> Result<Manifest>
	/// where
	/// 	<C as Client>::Error: Error + 'static
	/// {
	///   let json = client.get_json("https://piston-meta.mojang.com/v1/packages/d585c8e981e58326237746ca1253dea15c9e4aaa/24w21b.json".parse()?).await?;
	///   Ok(json)
	/// }
	/// ```
	async fn get_json<T>(&self, url: Url) -> Result<T, Self::Error>
	where
		T: DeserializeOwned;

	/// Sends a GET request and returns [AsyncRead] stream
	///
	/// # Example
	/// ```no_run
	/// # use std::error::Error;
	/// use async_compat::CompatExt;
	/// use futures_lite::io;
	/// use tokio::fs::File;
	/// use spuz_get::Client;
	/// use spuz_piston::Manifest;
	/// # type Result<T> = std::result::Result<T, Box<dyn Error>>;
	///
	/// async fn download_jar<C: Client>(client: &C) -> Result<()>
	/// where
	/// 	<C as Client>::Error: Error + 'static
	/// {
	///   let mut stream = client.get_stream("https://piston-data.mojang.com/v1/objects/9d5b45173a0123720bae94afc8a35d742e559d5a/client.jar".parse()?).await?;
	///   let mut file = File::open("./client.jar").await?;
	///   io::copy(&mut stream, &mut file.compat_mut()).await?;
	///   Ok(())
	/// }
	/// ```
	async fn get_stream(&self, url: Url) -> Result<BoxedAsyncRead, Self::Error>;
}

#[cfg(feature = "reqwest")]
#[async_trait]
impl Client for reqwest::Client {
	type Error = reqwest::Error;

	#[inline]
	async fn get_json<T>(&self, url: Url) -> Result<T, Self::Error>
	where
		T: DeserializeOwned,
	{
		self.get(url).send().await?.json().await
	}

	#[inline]
	async fn get_stream(&self, url: Url) -> Result<BoxedAsyncRead, Self::Error> {
		use futures_util::TryStreamExt;

		#[inline]
		fn map_err(err: reqwest::Error) -> std::io::Error {
			std::io::Error::new(std::io::ErrorKind::Other, err)
		}

		let bytes_stream = self.get(url).send().await?.bytes_stream().map_err(map_err);

		Ok(Box::new(bytes_stream.into_async_read()))
	}
}

#[derive(Debug, Error)]
pub enum FetchError<C: Client> {
	#[error("Fetch failed: {0}")]
	Client(#[source] C::Error),
	#[error("Invalid url: {0}")]
	ParseUrl(
		#[from]
		#[source]
		url::ParseError,
	),
}
