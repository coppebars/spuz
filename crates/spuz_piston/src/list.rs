use crate::{Arr, Str, UrlStr, VersionId, VersionStability};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latest {
	pub release: VersionId<Str>,
	pub snapshot: VersionId<Str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
	pub id: VersionId<Str>,
	pub r#type: VersionStability,
	pub sha1: Str,
	pub url: UrlStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
	latest: Latest,
	versions: Arr<Version>,
}
