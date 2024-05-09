use std::{
	iter,
	path::{Path, PathBuf},
};

use spuz_piston::{
	v::{
		shared::{Argument, Arguments, ListOrValue},
		AnyManifest,
	},
	Manifest, NativeClassifier, Rule, RuleCompilance, TARGET_OS,
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
		self.manifest.main_class().clone_into(launch_mod.main_class);

		match self.manifest.as_ref() {
			AnyManifest::V19(manifest) => {
				push_modern_arguments(self.rulecomp, &manifest.arguments, launch_mod);
			}
			AnyManifest::V12(manifest) => {
				push_modern_arguments(self.rulecomp, &manifest.arguments, launch_mod);
			}
		}

		let check_rule = |rule: &Rule| self.rulecomp.is_met(rule);

		let classpath: Box<dyn Iterator<Item = PathBuf>> = match self.manifest.as_ref() {
			AnyManifest::V19(manifest) => {
				let iter = manifest.libraries.iter().filter_map(|lib| {
					let lib_path = self.libraries_dir.join(&lib.downloads.artifact.path);

					let lib_rule_filter = |rules: &[Rule]| {
						let all_met = rules.iter().all(check_rule);
						all_met.then_some(lib_path.clone())
					};

					match &lib.rules {
						Some(rules) => lib_rule_filter(rules),
						None => Some(lib_path),
					}
				});

				Box::from(iter)
			}
			AnyManifest::V12(manifest) => {
				let iter = manifest
					.libraries
					.iter()
					.filter(|lib| match &lib.rules {
						Some(rules) => rules.iter().all(check_rule),
						None => true,
					})
					.flat_map(|lib| {
						let main_lib_path = self.libraries_dir.join(&lib.downloads.artifact.path);

						let target_classifier = NativeClassifier::from(TARGET_OS);
						if let Some(classifier) = lib.downloads.classifiers.as_ref().and_then(|it| it.get(&target_classifier)) {
							let native_lib_path = self.libraries_dir.join(&classifier.path);
							[Some(main_lib_path), Some(native_lib_path)]
						} else {
							[Some(main_lib_path), None]
						}
					})
					.flatten();

				Box::from(iter)
			}
		};

		let classpath = classpath.chain(iter::once(self.client_jar.to_owned()));

		let layers = (
			Classpath(classpath),
			VersionInfo {
				id: self.manifest.id(),
				version_type: self.manifest.version_type(),
				asset_index_id: self.manifest.asset_index_id(),
			},
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
