use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
	Release,
	Snapshot,
}

impl VersionType {
	pub fn is_release(&self) -> bool {
		matches!(self, Self::Release)
	}

	pub fn is_snapshot(&self) -> bool {
		!self.is_release()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionId(Box<str>);

impl Deref for VersionId {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[cfg(feature = "semver")]
impl VersionId {
	pub fn as_semver(&self) -> Option<semver::Version> {
		semver::Version::parse(&self.0).ok()
	}
}
