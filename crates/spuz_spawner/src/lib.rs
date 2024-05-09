use std::{fmt::Debug, path::PathBuf, process::Stdio};

use tokio::process::Command;
use tracing::debug;

pub use crate::{
	err::{Error, Result},
	layer::{LaunchMod, Layer},
	process::LaunchCommand,
};

pub mod compose;
mod err;
mod layer;
mod process;
#[cfg(feature = "useful-layers")]
pub mod useful;

#[derive(Debug, Default, Clone)]
pub struct CommandBuilder {
	bin: PathBuf,
	current_dir: PathBuf,
	main_class: String,
	java_args: Vec<String>,
	app_args: Vec<String>,
}

impl CommandBuilder {
	pub fn new(bin: impl Into<PathBuf>) -> Self {
		Self { bin: bin.into(), ..Default::default() }
	}

	pub fn apply(&mut self, layer: impl Layer + Debug) {
		debug!(?layer, "Applying layer");
		layer.apply(&mut LaunchMod {
			current_dir: &mut self.current_dir,
			java_args: &mut self.java_args,
			app_args: &mut self.app_args,
			main_class: &mut self.main_class,
		});
	}

	pub fn build(&self) -> LaunchCommand {
		let mut cmd = Command::new(&self.bin);
		cmd
			.current_dir(&self.current_dir)
			.args(&self.java_args) // Jvm args
			.arg(&self.main_class) // Main class
			.args(&self.app_args) // App args (minecraft args)
			.stdout(Stdio::piped()) // Pipe stdout
			.stderr(Stdio::piped()); // Pipe stderr
		debug!(?cmd, "Command built");
		LaunchCommand::new(cmd)
	}
}
