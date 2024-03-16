use serde::{Deserialize, Serialize};

use crate::common::{Str, Url, VersionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestSnapshotRelease {
	pub release: Str,
	pub snapshot: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingVersion {
	pub id: VersionId,
	pub r#type: Str,
	pub url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
	pub latest: LatestSnapshotRelease,
	pub versions: Vec<ListingVersion>,
}
