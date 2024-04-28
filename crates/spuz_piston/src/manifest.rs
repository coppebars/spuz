use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Artifact, ConditionalValue, Error, ListOrValue, PackageName, Rule, Size, UrlStr, Version};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
	Plain(String),
	Conditional(ConditionalValue<ListOrValue<String>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
	pub game: Vec<Argument>,
	pub jvm: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySpecifiers {
	pub artifact: Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
	pub name: PackageName,
	pub downloads: LibrarySpecifiers,
	pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResource {
	pub id: String,
	pub url: UrlStr,
	pub total_size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonManifest {
	/// Version info: id and stability
	#[serde(flatten)]
	pub version: Version,
	/// Asset index id
	pub assets: String,
	/// Reference to assets manifest
	pub asset_index: AssetIndexResource,
	/// Required or optional command-line arguments to configure game and player
	pub arguments: Arguments,
	/// Java libraries that minecraft uses
	pub libraries: Vec<Library>,
	/// Main class contains main method to start the game
	pub main_class: String,
}

impl FromStr for PistonManifest {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s).map_err(Into::into)
	}
}
