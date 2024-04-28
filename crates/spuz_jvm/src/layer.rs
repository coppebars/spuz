#[derive(Debug)]
pub struct LaunchMod<'a> {
	pub main_class: &'a mut String,
	pub java_args: &'a mut Vec<String>,
	pub app_args: &'a mut Vec<String>,
}

pub trait Layer {
	fn apply(self, launch_mod: &mut LaunchMod);
}
