use std::{collections::HashSet, iter, ops::Deref, path::Path, sync::Arc};

use spuz_jvm::{LaunchMod, Layer};
use spuz_piston::{Argument, Feature, ListOrValue, PistonManifest, Rule, RuleCompilance};

use crate::internal::{AssersDir, Classpath, GameDir, NativesDir, VersionInfo};

mod internal;
mod opts;

pub use crate::opts::{LauncherInfo, Player, WindowSize, Fullscreen};

#[derive(Debug)]
pub struct LauncherWrench {
	pub manifest: PistonManifest,
	pub libraries_dir: Arc<Path>,
	pub assets_dir: Arc<Path>,
	pub natives_dir: Arc<Path>,
	pub game_dir: Arc<Path>,
	pub client_jar: Arc<Path>,
	pub features: HashSet<Feature>,
}

impl Layer for LauncherWrench {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let rule_compilance = RuleCompilance::new(self.features);
		let check_rule = |rule| rule_compilance.is_met(&rule);

		*launch_mod.main_class = self.manifest.main_class;

		for arg in self.manifest.arguments.jvm {
			push_arg(&rule_compilance, launch_mod.java_args, arg);
		}

		for arg in self.manifest.arguments.game {
			push_arg(&rule_compilance, launch_mod.app_args, arg);
		}

		let classpath = self.manifest.libraries.into_iter().filter_map(|lib| {
			let lib_path = self.libraries_dir.join(lib.downloads.artifact.path);

			let lib_rule_filter = |rules: Vec<Rule>| {
				let all_met = rules.into_iter().all(check_rule);
				all_met.then_some(lib_path.clone())
			};

			match lib.rules {
				Some(rules) => lib_rule_filter(rules),
				None => Some(lib_path),
			}
		});

		let classpath = classpath.chain(iter::once(self.client_jar.deref().to_owned()));

		let layers = (
			Classpath(classpath),
			VersionInfo {
				id: &self.manifest.id,
				version_type: self.manifest.r#type.as_str(),
				asset_index_id: &self.manifest.asset_index.id,
			},
			AssersDir(&self.assets_dir),
			NativesDir(&self.natives_dir),
			GameDir(&self.game_dir),
		);

		layers.apply(launch_mod);
	}
}

fn push_arg(rule_compilance: &RuleCompilance, into: &mut Vec<String>, arg: Argument) {
	match arg {
		Argument::Plain(it) => {
			into.push(it);
		}
		Argument::Conditional(container) => {
			let arg = rule_compilance.unpack(container);

			if let Some(arg) = arg {
				match arg {
					ListOrValue::List(it) => {
						into.extend(it);
					}
					ListOrValue::Value(it) => {
						into.push(it);
					}
				}
			}
		}
	};
}
