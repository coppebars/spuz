pub mod shared;
pub mod v12;
pub mod v19;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use shared::{Arguments, Artifact, AssetIndexRef, Library};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum AnyManifest {
	V19(v19::RawManifest),
	V12(v12::RawManifest),
}

macro_rules! any {
	($self:ident, $($prop:tt)*) => {
		match $self {
			Self::V19(manifest) => &manifest.$($prop)*,
			Self::V12(manifest) => &manifest.$($prop)*,
		}
	};
}

impl AnyManifest {
	pub fn id(&self) -> &str {
		any!(self, id)
	}

	pub fn version_type(&self) -> &str {
		any!(self, r#type)
	}

	pub fn asset_index_id(&self) -> &str {
		any!(self, assets)
	}

	pub fn main_class(&self) -> &str {
		any!(self, main_class)
	}

	pub fn assets(&self) -> &AssetIndexRef {
		any!(self, asset_index)
	}

	pub fn release_time(&self) -> &str {
		any!(self, release_time)
	}
}

impl FromStr for AnyManifest {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}
