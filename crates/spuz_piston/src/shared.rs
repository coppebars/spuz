use std::{
	fmt::{Display, Formatter},
	ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::Result;

/// Type representing the size of the resource in bytes
pub type Size = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
#[serde(rename_all = "lowercase")]
pub enum Version {
	Release(String),
	Snapshot(String),
}

impl Version {
	pub fn is_release(&self) -> bool {
		matches!(self, Self::Release(_))
	}

	pub fn is_snapshot(&self) -> bool {
		!self.is_release()
	}

	pub fn version_type(&self) -> &'static str {
		match self {
			Version::Release(_) => "release",
			Version::Snapshot(_) => "snapshot",
		}
	}

	pub fn id(&self) -> &str {
		match self {
			Version::Snapshot(it) | Version::Release(it) => it,
		}
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Version::Release(semver) => write!(f, "{semver}"),
			Version::Snapshot(snapshot) => write!(f, "{snapshot}"),
		}
	}
}

impl From<Version> for String {
	fn from(value: Version) -> Self {
		match value {
			Version::Snapshot(it) | Version::Release(it) => it,
		}
	}
}

impl Deref for Version {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.id()
	}
}

/// This string is more than likely an [Url], but for memory and security
/// reasons you should lazily parse this string into an [Url].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlStr(String);

impl Deref for UrlStr {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for UrlStr {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl UrlStr {
	/// # Errors
	/// In case Url is invalid
	pub fn parse_url(&self) -> Result<Url> {
		Url::parse(&self.0).map_err(Into::into)
	}
}

/// A value that can be a list or a single value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrValue<T> {
	List(Vec<T>),
	Value(T),
}

impl<T> ListOrValue<T> {
	pub fn is_list(&self) -> bool {
		matches!(self, Self::List(_))
	}

	pub fn is_value(&self) -> bool {
		!self.is_list()
	}
}
