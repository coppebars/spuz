use crate::{UrlStr, VersionId, VersionStability};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latest {
	pub release: VersionId<String>,
	pub snapshot: VersionId<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
	pub id: VersionId<String>,
	pub r#type: VersionStability,
	pub sha1: String,
	pub url: UrlStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
	latest: Latest,
	versions: Vec<Version>,
}
