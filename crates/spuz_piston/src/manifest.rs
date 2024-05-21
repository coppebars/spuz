use std::{collections::HashMap, fmt::Debug, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{Arr, BoxPath, ConditionalValue, NativeClassifier, Rule, Size, Str};

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
}

impl FromStr for Manifest {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}
