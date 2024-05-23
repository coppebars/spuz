use std::{collections::HashMap, ops::Deref, str::FromStr};

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

use crate::{shared::BoxPath, Size, Str};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeTarget {
	#[serde(rename = "linux")]
	Linux,
	#[serde(rename = "linux-i386")]
	LinuxI386,
	#[serde(rename = "mac-os")]
	Macos,
	#[serde(rename = "mac-os-arm64")]
	MacosArm64,
	#[serde(rename = "windows-arm64")]
	WindowsArm64,
	#[serde(rename = "windows-x64")]
	WindowsX64,
	#[serde(rename = "windows-x86")]
	WindowsX86,
	#[serde(other)]
	GamecoreOrUnknown,
}

cfg_if! {
	if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::Linux;
	} else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::LinuxI386;
	} else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::Macos;
	} else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::MacosArm64;
	} else if #[cfg(all(target_os = "windows", target_arch = "aarch64"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::WindowsArm64;
	} else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::WindowsX64;
	} else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
		pub const TARGET_RUNTIME: RuntimeTarget = RuntimeTarget::WindowsX86;
	}
}

impl RuntimeTarget {
	pub fn is_target(self) -> bool {
		self == TARGET_RUNTIME
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Component {
	#[serde(rename = "java-runtime-alpha")]
	Alpha,
	#[serde(rename = "java-runtime-beta")]
	Beta,
	#[serde(rename = "java-runtime-delta")]
	Delta,
	#[serde(rename = "java-runtime-gamma")]
	Gamma,
	#[serde(rename = "java-runtime-gamma-snapshot")]
	GammaSnapshot,
	#[serde(rename = "jre-legacy")]
	Legacy,
	#[serde(rename = "minecraft-java-exe")]
	Exe,
}

impl FromStr for Component {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"java-runtime-alpha" => Ok(Self::Alpha),
			"java-runtime-beta" => Ok(Self::Beta),
			"java-runtime-delta" => Ok(Self::Delta),
			"java-runtime-gamma" => Ok(Self::Gamma),
			"java-runtime-gamma-snapshot" => Ok(Self::GammaSnapshot),
			"jre-legacy" => Ok(Self::Legacy),
			"minecraft-java-exe" => Ok(Self::Exe),
			_ => Err(()),
		}
	}
}

mod private {
	pub(super) trait SealedParseComponent {}
	impl<T: AsRef<str>> SealedParseComponent for T {}
}

pub trait ParseComponent {
	fn parse_component(&self) -> Option<Component>;
}

impl<T: AsRef<str> + private::SealedParseComponent> ParseComponent for T {
	fn parse_component(&self) -> Option<Component> {
		self.as_ref().parse().ok()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilityInfo {
	pub group: u32,
	pub progress: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionInfo {
	pub name: Str,
	pub released: Str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestInfo {
	pub url: Str,
	pub size: Size,
	pub sha1: Str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentInfo {
	pub availability: AvailabilityInfo,
	pub version: VersionInfo,
	pub manifest: ManifestInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentInfoItem {
	pub inner: Box<[ComponentInfo]>,
}

impl Deref for ComponentInfoItem {
	type Target = [ComponentInfo];

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentList {
	pub inner: HashMap<Component, ComponentInfoItem>,
}

impl ComponentList {
	pub fn get(&self, component: &impl ParseComponent) -> Option<&ComponentInfo> {
		component.parse_component().and_then(|it| self.inner.get(&it).and_then(|it| it.first()))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RuntimeComponents {
	pub targets: HashMap<RuntimeTarget, ComponentList>,
}

impl Deref for RuntimeComponents {
	type Target = HashMap<RuntimeTarget, ComponentList>;

	fn deref(&self) -> &Self::Target {
		&self.targets
	}
}

impl RuntimeComponents {
	pub fn target(&self) -> Option<&ComponentList> {
		self.targets.get(&TARGET_RUNTIME)
	}

	pub fn component(&self, component: &impl ParseComponent) -> Option<&ComponentInfo> {
		self.target().and_then(|it| it.get(component))
	}
}

impl FromStr for RuntimeComponents {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFile {
	pub sha1: Str,
	pub url: Str,
	pub size: Size,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeFileDownloads {
	pub raw: RuntimeFile,
	pub lzma: Option<RuntimeFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum RuntimeSource {
	File { downloads: RuntimeFileDownloads, executable: bool },
	Link { target: BoxPath },
	Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeManifest {
	files: HashMap<BoxPath, RuntimeSource>,
}

impl Deref for RuntimeManifest {
	type Target = HashMap<BoxPath, RuntimeSource>;

	fn deref(&self) -> &Self::Target {
		&self.files
	}
}

impl FromStr for RuntimeManifest {
	type Err = serde_json::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}
