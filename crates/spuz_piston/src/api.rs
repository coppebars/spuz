use std::fmt::Display;

use reqwest::{Client, Response};
use serde::Deserialize;
use url::Url;

use crate::{list::Versions, Result};

#[derive(Debug)]
pub struct Api {
	client: Client,
}

impl Api {
	pub fn new(client: Client) -> Self {
		Self { client }
	}

	pub async fn list(&self) -> Result<Versions> {
		let url = Url::parse("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")?;

		let request = self.client.get(url);
		let response = request.send().await?;
		let list = response.json().await?;

		Ok(list)
	}

	pub async fn package_file<T>(&self, hash: T, id: T) -> Result<Response>
	where
		T: Display,
	{
		let url = format!("https://piston-meta.mojang.com/v1/packages/{hash}/{id}.json");
		let url = Url::parse(&url)?;

		let request = self.client.get(url);
		let response = request.send().await?;
		Ok(response)
	}

	pub async fn package<U, T>(&self, hash: T, id: T) -> Result<U>
	where
		T: Display,
		U: for<'de> Deserialize<'de>,
	{
		let response = self.package_file(hash, id).await?;
		let package = response.json().await?;

		Ok(package)
	}
}
