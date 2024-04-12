use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Os {
	Linux,
	Windows,
	Osx,
}

cfg_if! {
	if #[cfg(target_os = "windows")] {
		pub const TARGET_OS: Os = Os::Windows;
	} else if #[cfg(target_os = "linux")] {
		pub const TARGET_OS: Os = Os::Linux;
	} else if #[cfg(target_os = "macos")] {
		pub const TARGET_OS: Os = Os::Osx;
	} else {
		compile_error!("Sorry, Your OS is not supported");
	}
}

impl Os {
	pub fn is_target(self) -> bool {
		self == TARGET_OS
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
	X64,
	X86,
}

cfg_if! {
	if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
		pub const TARGET_ARCH: Arch = Arch::X64;
	} else if #[cfg(any(target_arch = "x86", target_arch = "arm"))] {
		pub const TARGET_ARCH: Arch = Arch::X86;
	} else {
		compile_error!("Sorry, Your CPU arch is not supported");
	}
}

impl Arch {
	pub fn is_target(self) -> bool {
		self == TARGET_ARCH
	}
}
