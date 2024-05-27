mod internal;
mod mandep;
mod opts;

use std::{collections::HashSet, path::Path};

use spuz_piston::{
	rule::{Feature, RuleCompilance},
	Manifest,
};
use spuz_spawner::{LaunchMod, Layer};
use typed_builder::TypedBuilder;

pub use crate::opts::{Fullscreen, LauncherInfo, Player, WindowSize};
use crate::{
	internal::{AssersDir, GameDir, NativesDir},
	mandep::ManifestLayer,
};

#[derive(Debug, TypedBuilder)]
pub struct LauncherWrench<'a> {
	pub manifest: &'a Manifest,
	pub current_dir: &'a Path,
	#[builder(default, setter(into))]
	pub libraries_dir: Option<&'a Path>,
	#[builder(default, setter(into))]
	pub assets_dir: Option<&'a Path>,
	#[builder(default, setter(into))]
	pub natives_dir: Option<&'a Path>,
	pub game_dir: &'a Path,
	#[builder(default, setter(into))]
	pub client_jar: Option<&'a Path>,
	#[builder(default = HashSet::from([Feature::CustomResolution]))]
	pub features: HashSet<Feature>,
}

impl<'a> Layer for LauncherWrench<'a> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let rule_compilance = RuleCompilance::new(self.features);

		self.current_dir.clone_into(launch_mod.current_dir);

		let id = &self.manifest.id;
		let client_dir = Path::new("versions").join(&**id);
		let client_jar = client_dir.join(format!("{id}.jar"));
		let client_natives = client_dir.join("natives");

		let layers = (
			ManifestLayer {
				rulecomp: &rule_compilance,
				manifest: self.manifest,
				libraries_dir: self.libraries_dir.unwrap_or(Path::new("libraries")),
				client_jar: self.client_jar.unwrap_or(&client_jar),
			},
			AssersDir(self.assets_dir.unwrap_or(Path::new("assets"))),
			NativesDir(self.natives_dir.unwrap_or(&client_natives)),
			GameDir(self.game_dir),
		);

		layers.apply(launch_mod);
	}
}
