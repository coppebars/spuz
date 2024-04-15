use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Artifact, ConditionalValue, ListOrValue, PackageName, Rule, Size, UrlStr, VersionId, VersionStability};

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
pub struct PistonPackage {
	pub r#type: VersionStability,
	pub id: VersionId<String>,
	pub asset_index: AssetIndexResource,
	pub arguments: Arguments,
	pub libraries: Vec<Library>,
}

impl FromStr for Box<PistonPackage> {
	type Err = serde_json::error::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}
