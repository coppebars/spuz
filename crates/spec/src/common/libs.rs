use std::{
	fmt::{Display, Formatter},
	path::{Component, Path, PathBuf},
};

use cfg_if::cfg_if;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use super::{Sha, Size, Str, Url};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageName {
	pub package: PathBuf,
	pub name: Str,
	#[cfg(feature = "semver")]
	pub version: semver::Version,
	#[cfg(not(feature = "semver"))]
	pub version: Str,
}

impl Display for PackageName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for (idx, component) in self.package.components().enumerate() {
			if let Component::Normal(name) = component {
				if idx > 0 {
					write!(f, ".")?;
				}
				write!(f, "{}", name.to_string_lossy())?;
			} else {
				write!(f, "<invalid>")?;
			}
		}

		write!(f, ":{}:{}", self.name, self.version)
	}
}

impl Serialize for PackageName {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&format!("{self}"))
	}
}

impl<'de> Deserialize<'de> for PackageName {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct ArtifactNameVisitor;

		impl<'vis> Visitor<'vis> for ArtifactNameVisitor {
			type Value = PackageName;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				write!(formatter, "a string in java package format `org.name.pkg:artifact:version`")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				let parts: Vec<&str> = v.split(':').collect();

				cfg_if! {
					if #[cfg(feature = "semver")] {
						Ok(PackageName {
							package: parts[0].split('.').collect(),
							name: parts[1].into(),
							version: semver::Version::parse(parts[2]).unwrap_or(semver::Version::new(0, 0, 0)),
						})
					} else {
						Ok(PackageName {
							package: parts[0].split('.').collect(),
							name: parts[1].into(),
							version: parts[2].into(),
						})
					}
				}
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				self.visit_str(&v)
			}
		}

		deserializer.deserialize_string(ArtifactNameVisitor)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
	pub path: Box<Path>,
	pub sha1: Sha,
	pub size: Size,
	pub url: Url,
}
