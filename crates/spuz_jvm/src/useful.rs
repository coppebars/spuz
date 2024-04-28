use std::{fmt::Display, ops::Range};

use tracing::debug;

use crate::{LaunchMod, Layer};

#[derive(Debug, Clone)]
pub struct AllocRange<T>(pub Range<T>);

impl<T> Layer for AllocRange<T>
where
	T: Display,
{
	fn apply(self, launch_mod: &mut LaunchMod) {
		let min = self.0.start;
		let max = self.0.end;
		launch_mod.java_args.push(format!("-Xms{min}m"));
		launch_mod.java_args.push(format!("-Xmx{max}m"));
		debug!(%min, %max, "Allocation range arguments are set. Min(xms): {min}Mb. Max(xmx): {max}Mb");
	}
}

#[derive(Debug, Clone)]
pub struct Dparam<Name, Value>(pub Name, pub Value);

impl<Name, Value> Layer for Dparam<Name, Value>
where
	Name: Display,
	Value: Display,
{
	fn apply(self, launch_mod: &mut LaunchMod) {
		let Self (name, value) = self;
		let arg = format!("-D{name}={value}");
		debug!(?arg, "D-parameter with name: `{name}` set to value: `{value}`");
		launch_mod.java_args.push(arg);
	}
}

#[derive(Debug)]
pub struct JavaArg<T>(pub T);

impl<T> Layer for JavaArg<T>
where
	T: Display,
{
	fn apply(self, launch_mod: &mut LaunchMod) {
		debug!(arg = %self.0, "Java argument added: `{}`", self.0);
		launch_mod.java_args.push(self.0.to_string());
	}
}

#[derive(Debug)]
pub struct AppArg<T>(pub T);

impl<T> Layer for AppArg<T>
where
	T: Display,
{
	fn apply(self, launch_mod: &mut LaunchMod) {
		debug!("Application argument added: `{}`", self.0);
		launch_mod.app_args.push(self.0.to_string());
	}
}
