use serde::{Deserialize, Serialize};

use crate::common::{
	Artifact, ConditionalValue, ListOrValue, PackageName, Rule, Size, Str, Url, VersionId, VersionType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
	Plain(Str),
	Conditional(ConditionalValue<ListOrValue<Str>>),
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
pub struct AssetIndex {
	pub id: Str,
	pub url: Url,
	pub total_size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientVersion {
	pub r#type: VersionType,
	pub id: VersionId,
	pub asset_index: AssetIndex,
	pub libraries: Vec<Library>,
	pub arguments: Arguments,
}
