use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Os {
	Windows,
	Linux,
	Osx,
}

impl Os {
	pub fn target() -> Self {
		cfg_if! {
			if #[cfg(target_os = "linux")] {
				Self::Linux
			} else if #[cfg(target_os = "windows")] {
				Self::Windows
			} else if #[cfg(target_os = "macos")] {
				Self::Osx
			} else {
				compile_error!("Your target os is not supported");
			}
		}
	}

	pub fn is_target(self) -> bool {
		Self::target() == self
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
	X64,
	X86,
}

impl Arch {
	pub fn target() -> Self {
		cfg_if! {
			if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
				Self::X64
			} else if #[cfg(any(target_arch = "x86", target_arch = "arm"))] {
				Self::X86
			} else {
				compile_error!("Your target arch is not supported");
			}
		}
	}

	pub fn is_target(self) -> bool {
		Self::target() == self
	}
}
