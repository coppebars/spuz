use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Sha, Size, Str};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetObject {
	pub hash: Sha,
	pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
	pub objects: HashMap<Str, AssetObject>,
}
