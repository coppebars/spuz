use std::{
	collections::HashMap,
	fmt::{Debug, Display, Formatter},
	str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{Size, Str};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetObject {
	pub hash: Str,
	pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetIndex {
	pub objects: HashMap<Str, AssetObject>,
}

impl FromStr for AssetIndex {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

impl Display for AssetIndex {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		serde_json::to_string(self).map_err(|_| std::fmt::Error).fmt(f)
	}
}
