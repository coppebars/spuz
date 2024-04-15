use itertools::Itertools;
use spuz_jvm::Jvm;
use spuz_piston::{Argument, Feature, ListOrValue, PistonPackage, Rule, RuleCompilance};
use std::collections::HashSet;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

#[cfg(target_os = "windows")]
const SEP: &str = ";";
#[cfg(not(target_os = "windows"))]
const SEP: &str = ":";

#[derive(Debug, Copy, Clone)]
pub struct Resolution {
	pub width: u32,
	pub height: u32,
}

impl Resolution {
	pub fn new(width: u32, height: u32) -> Self {
		Self { width, height }
	}
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Launcher {
	bin: PathBuf,
	package: Box<PistonPackage>,
	player_name: String,
	libraries_dir: PathBuf,
	game_dir: PathBuf,
	assets_dir: PathBuf,
	uuid: Option<String>,
	access_token: Option<String>,
	client_id: Option<String>,
	xuid: Option<String>,
	resolution: Option<Resolution>,
	natives_dir: PathBuf,
	launcher_name: Option<String>,
	launcher_version: Option<String>,
	features: HashSet<Feature>,
}

impl Launcher {
	pub fn compile(self) -> Jvm {
		let mut jvm = Jvm::new(self.bin);

		jvm.var("version_name", self.package.id.to_string());
		jvm.var("version_type", self.package.r#type.as_str());
		jvm.var("assets_index_name", self.package.asset_index.id);
		jvm.var("user_type", "msa");
		jvm.var("game_directory", self.game_dir.to_string_lossy());
		jvm.var("assets_root", self.assets_dir.to_string_lossy());

		jvm.var("auth_player_name", self.player_name);
		jvm.var_opt("auth_uuid", self.uuid);
		jvm.var_opt("clientid", self.client_id);
		jvm.var_opt("auth_xuid", self.xuid);

		if let Some(resolution) = self.resolution {
			jvm.var("resolution_width", resolution.width.to_string());
			jvm.var("resolution_height", resolution.height.to_string());
		}

		let rule_compilance = RuleCompilance::new(self.features);
		let check_rule = |rule| rule_compilance.is_met(&rule);

		let classpath = self.package.libraries.into_iter().filter_map(|lib| {
			let lib_rule_filter = |rules: Vec<Rule>| {
				let all_met = rules.into_iter().all(check_rule);
				let lib_path = self.libraries_dir.join(lib.downloads.artifact.path);
				all_met.then_some(lib_path)
			};

			lib.rules.and_then(lib_rule_filter)
		});

		let stringify_path = |it: PathBuf| it.to_string_lossy().into_owned();

		let cp_string = classpath.map(stringify_path).join(SEP);

		jvm.var("classpath", cp_string);

		let arg_map = |arg: Argument| match arg {
			Argument::Plain(it) => Some(vec![it]),
			Argument::Conditional(it) => rule_compilance.unpack(it).map(|it| match it {
				ListOrValue::List(it) => it,
				ListOrValue::Value(it) => vec![it],
			}),
		};

		let jargs = self.package.arguments.jvm.into_iter().filter_map(arg_map).flatten();
		let aargs = self.package.arguments.game.into_iter().filter_map(arg_map).flatten();

		jvm.jargs.extend(jargs);
		jvm.aargs.extend(aargs);

		jvm
	}
}
