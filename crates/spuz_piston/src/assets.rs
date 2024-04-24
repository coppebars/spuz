use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Size;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetObject {
	pub hash: String,
	pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
	pub objects: HashMap<String, AssetObject>,
}
