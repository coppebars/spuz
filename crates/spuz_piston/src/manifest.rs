use std::{fmt::Debug, ops::Deref, str::FromStr};

use crate::v::AnyManifest;

#[derive(Debug, Clone)]
pub struct Manifest {
	raw: AnyManifest,
}

impl Manifest {
	fn new(raw: AnyManifest) -> Self {
		Self { raw }
	}
}

impl Deref for Manifest {
	type Target = AnyManifest;

	fn deref(&self) -> &Self::Target {
		&self.raw
	}
}

impl AsRef<AnyManifest> for Manifest {
	fn as_ref(&self) -> &AnyManifest {
		&self.raw
	}
}

impl FromStr for Manifest {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::new(s.parse()?))
	}
}
