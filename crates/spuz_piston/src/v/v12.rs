use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Arguments, Artifact, AssetIndexRef, Library};
use crate::{Arr, NativeClassifier, Str};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specifiers {
	pub artifact: Artifact,
	pub classifiers: Option<HashMap<NativeClassifier, Artifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawManifest {
	pub r#type: Str,
	pub id: Str,
	pub time: Str,
	pub release_time: Str,
	pub main_class: Str,
	pub assets: Str,
	pub asset_index: AssetIndexRef,
	pub libraries: Arr<Library<Specifiers>>,
	pub arguments: Arguments,
}
