use std::collections::HashMap;

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
