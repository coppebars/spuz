use std::{
	collections::HashMap,
	fmt::{Debug, Display},
	ops::Deref,
	str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
	platform::NativeClassifier,
	rule::{ConditionalValue, Rule},
	Arr, BoxPath, Size, Str,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ListOrValue<T> {
	List(Arr<T>),
	Value(T),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRef {
	pub id: Str,
	pub url: Str,
	pub total_size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artifact {
	pub path: BoxPath,
	pub sha1: Str,
	pub size: Size,
	pub url: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Argument {
	Plain(Str),
	Conditional(ConditionalValue<ListOrValue<Str>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Arguments {
	pub game: Arr<Argument>,
	pub jvm: Arr<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Library {
	pub name: Str,
	pub downloads: Specifiers,
	pub rules: Option<Arr<Rule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specifiers {
	pub artifact: Option<Artifact>,
	pub classifiers: Option<HashMap<NativeClassifier, Artifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadItem {
	pub sha1: Str,
	pub size: Size,
	pub url: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Downloads {
	pub client: DownloadItem,
	pub client_mappings: Option<DownloadItem>,
	pub server: DownloadItem,
	pub server_mappings: Option<DownloadItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoggingFile {
	pub id: Str,
	pub sha1: Str,
	pub size: Size,
	pub url: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientLogging {
	pub argument: Str,
	pub file: LoggingFile,
	pub r#type: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Logging {
	pub client: ClientLogging,
}

impl Deref for Logging {
	type Target = ClientLogging;

	fn deref(&self) -> &Self::Target {
		&self.client
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
	pub r#type: Str,
	pub id: Str,
	pub time: Str,
	pub release_time: Str,
	pub main_class: Str,
	pub assets: Str,
	pub asset_index: AssetIndexRef,
	pub libraries: Arr<Library>,
	pub arguments: Arguments,
	pub downloads: Downloads,
	pub logging: Logging,
}

impl FromStr for Manifest {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

impl Display for Manifest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		serde_json::to_string(self).map_err(|_| std::fmt::Error).fmt(f)
	}
}
