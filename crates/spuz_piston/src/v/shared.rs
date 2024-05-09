use serde::{Deserialize, Serialize};

use crate::{Arr, BoxPath, ConditionalValue, Rule, Size, Str};

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
pub struct Library<S> {
	pub name: Str,
	pub downloads: S,
	pub rules: Option<Arr<Rule>>,
}
