use serde::{Deserialize, Serialize};

use crate::{
	Arr, Artifact, ConditionalValue, ListOrValue, PackageName, Rule, Size, Str, UrlStr, VersionId, VersionStability,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
	Plain(Str),
	Conditional(ConditionalValue<ListOrValue<Str>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
	pub game: Arr<Argument>,
	pub jvm: Arr<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySpecifiers {
	pub artifact: Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
	pub name: PackageName,
	pub downloads: LibrarySpecifiers,
	pub rules: Option<Arr<Rule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResource {
	pub id: Str,
	pub url: UrlStr,
	pub total_size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonPackage {
	pub r#type: VersionStability,
	pub id: VersionId<Str>,
	pub asset_index: AssetIndexResource,
	pub arguments: Arguments,
}
