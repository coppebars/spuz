use std::{
	fmt::{Display, Formatter},
	ops::Deref,
	str::FromStr,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::Result;

/// Type representing the size of the resource in bytes
pub type Size = u64;

/// Version stability is determined by whether a version is a release or a
/// snapshot.
///
/// # Note
/// Legacy definitions like `old_alpha` and `old_beta` are not supported
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStability {
	/// Stable version (release or prerelease)
	Release,
	/// Work in progress (aka. snapshot) version
	Snapshot,
	/// Unknown/Unsupported
	#[serde(other)]
	Unknown,
}

impl VersionStability {
	/// Is this version a release
	pub fn is_release(self) -> bool {
		matches!(self, Self::Release)
	}

	/// Is this version a snapshot
	pub fn is_snapshot(self) -> bool {
		!self.is_release()
	}
}

impl FromStr for VersionStability {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"release" => Ok(Self::Release),
			"snapshot" => Ok(Self::Snapshot),
			_ => Err(()),
		}
	}
}

impl VersionStability {
	pub fn as_str(self) -> &'static str {
		match self {
			VersionStability::Release => "release",
			VersionStability::Snapshot => "snapshot",
			VersionStability::Unknown => panic!("VersionStability must not be used with legacy versions"),
		}
	}
}

// TODO: now it's not used
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SemanticVersion {
	Release(semver::Version),
	Snapshot { year: u8, week: u8, suffix: String },
}

impl Display for SemanticVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SemanticVersion::Release(semver) => write!(f, "{semver}"),
			SemanticVersion::Snapshot { year, week, suffix } => write!(f, "{year}w{week}{suffix}"),
		}
	}
}

/// The version identifier can represent either semver (which is specific to a
/// release) or a sequential snapshot number.
///
/// # Todo
/// Only string representation is available now
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionId<T>(T);

impl<T> Deref for VersionId<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
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

impl UrlStr {
	/// # Errors
	/// In case Url is invalid
	pub fn parse_url(&self) -> Result<Url> {
		Url::parse(&self.0).map_err(Into::into)
	}

	/// # Panics
	/// In case Url is invalid
	pub fn parse_url_unchecked(&self) -> Url {
		self.parse_url().expect("Invalid Url")
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
