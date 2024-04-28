use std::path::Path;

use cfg_if::cfg_if;
use itertools::Itertools;
use spuz_jvm::{LaunchMod, Layer};
use tracing::debug;

cfg_if! {
	if #[cfg(target_os = "windows")] {
		const SEP: &str = ";";
	} else {
		const SEP: &str = ":";
	}
}

#[derive(Debug, Default, Clone)]
pub struct Classpath<T>(pub T);

impl<T> From<T> for Classpath<T> {
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T, A> Layer for Classpath<T>
where
	T: Iterator<Item = A>,
	A: AsRef<Path>,
{
	fn apply(self, launch_mod: &mut LaunchMod) {
		let mut len = 0;
		let cp = self
			.0
			.map(|it| {
				len += 1;
				it.as_ref().to_string_lossy().into_owned()
			})
			.join(SEP);
		for arg in &mut *launch_mod.java_args {
			*arg = arg.replace("${classpath}", &cp);
		}
		debug!("{len} items added to the classpath");
	}
}

#[derive(Debug)]
pub struct NativesDir<'a>(pub &'a Path);

impl<'a> Layer for NativesDir<'a> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let str_path = self.0.to_string_lossy();
		for arg in &mut *launch_mod.java_args {
			*arg = arg.replace("${natives_directory}", &str_path);
		}
		debug!("Library(natives) path set to {:?}", self.0);
	}
}

#[derive(Debug)]
pub struct VersionInfo<'a> {
	pub id: &'a str,
	pub version_type: &'a str,
	pub asset_index_id: &'a str,
}

impl Layer for VersionInfo<'_> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		for arg in &mut *launch_mod.app_args {
			*arg = arg.replace("${version_name}", self.id);
			*arg = arg.replace("${version_type}", self.version_type);
			*arg = arg.replace("${user_type}", "msa");
			*arg = arg.replace("${assets_index_name}", self.asset_index_id);
		}
	}
}

#[derive(Debug)]
pub struct AssersDir<'a>(pub &'a Path);

impl<'a> Layer for AssersDir<'a> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let str_path = self.0.to_string_lossy();
		for arg in &mut *launch_mod.app_args {
			*arg = arg.replace("${assets_root}", &str_path);
		}
		debug!("Assets directory set to {:?}", &self.0);
	}
}

#[derive(Debug)]
pub struct GameDir<'a>(pub &'a Path);

impl<'a> Layer for GameDir<'a> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let str_path = self.0.to_string_lossy();
		for arg in &mut *launch_mod.app_args {
			*arg = arg.replace("${game_directory}", &str_path);
		}
		debug!("Game(instance) directory set to {:?}", &self.0);
	}
}
