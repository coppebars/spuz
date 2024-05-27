use std::{iter, path::Path};

use spuz_piston::{
	manifest::{Argument, Arguments, ListOrValue},
	platform::{NativeClassifier, TARGET_OS},
	rule::{Rule, RuleCompilance},
	Manifest,
};
use spuz_spawner::{LaunchMod, Layer};

use crate::internal::{Classpath, VersionInfo};

#[derive(Debug)]
pub struct ManifestLayer<'a> {
	pub rulecomp: &'a RuleCompilance,
	pub manifest: &'a Manifest,
	pub libraries_dir: &'a Path,
	pub client_jar: &'a Path,
}

impl<'a> Layer for ManifestLayer<'a> {
	fn apply(self, launch_mod: &mut LaunchMod) {
		self.manifest.main_class.as_ref().clone_into(launch_mod.main_class);

		push_modern_arguments(self.rulecomp, &self.manifest.arguments, launch_mod);

		let check_rule = |rule: &Rule| self.rulecomp.is_met(rule);

		let iter = self
			.manifest
			.libraries
			.iter()
			.filter(|lib| match &lib.rules {
				Some(rules) => rules.iter().all(check_rule),
				None => true,
			})
			.flat_map(|lib| {
				let main_lib_path = lib.downloads.artifact.as_ref().map(|it| self.libraries_dir.join(&it.path));

				let target_classifier = NativeClassifier::from(TARGET_OS);
				if let Some(classifier) = lib.downloads.classifiers.as_ref().and_then(|it| it.get(&target_classifier)) {
					let native_lib_path = self.libraries_dir.join(&classifier.path);
					[main_lib_path, Some(native_lib_path)]
				} else {
					[main_lib_path, None]
				}
			})
			.flatten();

		let classpath = iter.chain(iter::once(self.client_jar.to_owned()));

		let layers = (
			Classpath(classpath),
			VersionInfo { id: &self.manifest.id, version_type: &self.manifest.r#type, asset_index_id: &self.manifest.assets },
		);

		layers.apply(launch_mod);
	}
}

fn push_modern_arguments(rulcomp: &RuleCompilance, args: &Arguments, launch_mod: &mut LaunchMod) {
	for arg in args.jvm.iter() {
		push_modern_arg(rulcomp, launch_mod.java_args, arg);
	}

	for arg in args.game.iter() {
		push_modern_arg(rulcomp, launch_mod.app_args, arg);
	}
}

fn push_modern_arg(rule_compilance: &RuleCompilance, into: &mut Vec<String>, arg: &Argument) {
	match arg {
		Argument::Plain(it) => {
			into.push(it.to_string());
		}
		Argument::Conditional(container) => {
			let arg = rule_compilance.unpack_ref(container);

			if let Some(arg) = arg {
				match arg {
					ListOrValue::List(it) => {
						into.extend(it.iter().map(ToString::to_string));
					}
					ListOrValue::Value(it) => {
						into.push(it.to_string());
					}
				}
			}
		}
	};
}
