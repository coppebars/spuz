use serde::{Deserialize, Serialize};

use crate::UrlStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latest {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRef {
	#[serde(flatten)]
	pub version: crate::Version,
	pub sha1: String,
	pub url: UrlStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
	latest: Latest,
	versions: Vec<VersionRef>,
}
