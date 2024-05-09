use serde::{Deserialize, Serialize};

use crate::{Arr, Str};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latest {
	pub release: Str,
	pub snapshot: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRef {
	pub r#type: Str,
	pub id: Str,
	pub sha1: Str,
	pub url: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
	latest: Latest,
	versions: Arr<VersionRef>,
}
