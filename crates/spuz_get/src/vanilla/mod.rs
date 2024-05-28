use std::marker::PhantomData;

use spuz_piston::list::Versions;
use url::Url;

use crate::{client::BoxedAsyncRead, Client, FetchError, JsonResource};

/// Lists all versions of minecraft over time
pub async fn list<C: Client>(client: &C) -> Result<Versions, FetchError<C>> {
	let url = Url::parse("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")?;
	client.get_json(url).await.map_err(FetchError::Client)
}

/// Requests json package from `https://piston-meta.mojang.com/v1/packages` by `hash` and `id`
pub async fn package<P, C: Client>(
	client: &C,
	hash: &str,
	id: &str,
) -> Result<JsonResource<BoxedAsyncRead, P>, FetchError<C>> {
	let url: Url = format!("https://piston-meta.mojang.com/v1/packages/{hash}/{id}.json").parse()?;
	let stream = client.get_stream(url).await.map_err(FetchError::Client)?;

	Ok(JsonResource { stream, json: PhantomData })
}

/// Requests binary object from `https://piston-meta.mojang.com/v1/objects` by `hash` and `id`
pub async fn object<C: Client>(client: &C, hash: &str, id: &str) -> Result<BoxedAsyncRead, FetchError<C>> {
	let url: Url = format!("https://piston-meta.mojang.com/v1/objects/{hash}/{id}").parse()?;
	let stream = client.get_stream(url).await.map_err(FetchError::Client)?;

	Ok(stream)
}

/// Requests game resource (aka assets) from `https://resources.download.minecraft.net/`
pub async fn resource<C: Client>(client: &C, hash: &str) -> Result<BoxedAsyncRead, FetchError<C>> {
	let url: Url = format!("https://resources.download.minecraft.net/{h2}/{hash}", h2 = &hash[..2]).parse()?;
	let stream = client.get_stream(url).await.map_err(FetchError::Client)?;

	Ok(stream)
}
