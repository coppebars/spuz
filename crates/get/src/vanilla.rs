use std::future::Future;

use reqwest::{IntoUrl, Response};
use spuz_spec::vanilla::{assets::AssetIndex, client::latest::ClientVersion, listing::Listing};
use url::Url;

use crate::{
	endpoints::{MOJANG_ASSETS, MOJANG_VERSION_LISTING},
	Fetch, Result,
};

pub trait VanillaFetch {
	fn vanilla_listing(&self) -> impl Future<Output = Result<Box<Listing>>> + Send;
	fn vanilla_client_version(&self, url: impl IntoUrl + Send)
	-> impl Future<Output = Result<Box<ClientVersion>>> + Send;
	fn vanilla_asset_index(&self, url: impl IntoUrl + Send) -> impl Future<Output = Result<Box<AssetIndex>>> + Send;
	fn vanilla_asset(&self, hash: impl ToString) -> impl Future<Output = Result<Response>>;
}

impl VanillaFetch for Fetch {
	async fn vanilla_listing(&self) -> Result<Box<Listing>> {
		Ok(self.client.get(MOJANG_VERSION_LISTING).send().await?.json().await?)
	}

	async fn vanilla_client_version(&self, url: impl IntoUrl + Send) -> Result<Box<ClientVersion>> {
		Ok(self.client.get(url).send().await?.json().await?)
	}

	async fn vanilla_asset_index(&self, url: impl IntoUrl + Send) -> Result<Box<AssetIndex>> {
		Ok(self.client.get(url).send().await?.json().await?)
	}

	async fn vanilla_asset(&self, hash: impl ToString) -> Result<Response> {
		let hash = hash.to_string();
		Ok(self.client.get(Url::parse(MOJANG_ASSETS).unwrap().join(&hash[..2]).unwrap().join(&hash).unwrap()).send().await?)
	}
}
