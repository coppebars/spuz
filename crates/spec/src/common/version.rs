use std::ops::Deref;

use serde::{Deserialize, Serialize};
use crate::common::Str;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
	Release,
	Snapshot,
}

impl VersionType {
	pub fn is_release(self) -> bool {
		matches!(self, Self::Release)
	}

	pub fn is_snapshot(self) -> bool {
		!self.is_release()
	}
}

pub mod format {
	use crate::common::Str;

	pub trait VersionFormat {}
	#[cfg(feature = "semver")]
	impl VersionFormat for semver::Version {}
	impl VersionFormat for Str {}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionId<T: format::VersionFormat = Str>(T);

impl<T: format::VersionFormat> Deref for VersionId<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[cfg(feature = "semver")]
impl VersionId<Str> {
	pub fn to_semver(&self) -> Option<VersionId<semver::Version>> {
		semver::Version::parse(&self.0).ok().map(VersionId)
	}
}
