use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Str;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
	pub r#type: Str,
	pub name: Str,
	pub created: Str,
	pub icon: Str,
	pub last_used: Str,
	pub last_version_id: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherProfiles {
	pub profiles: HashMap<Str, Profile>,
	pub version: u32,
}
