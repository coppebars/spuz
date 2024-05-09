use spuz_spawner::{LaunchMod, Layer};
use tracing::debug;

#[derive(Debug, Clone, Default)]
pub struct Player {
	pub username: String,
	pub uuid: String,
	pub access_token: Option<String>,
	pub client_id: Option<String>,
	pub xuid: Option<String>,
}

impl Player {
	pub fn new(username: impl Into<String>, uuid: impl Into<String>) -> Self {
		Self { username: username.into(), uuid: uuid.into(), ..Default::default() }
	}
}

impl Layer for Player {
	fn apply(self, launch_mod: &mut LaunchMod) {
		for arg in &mut *launch_mod.app_args {
			*arg = arg.replace("${auth_player_name}", &self.username);
			*arg = arg.replace("${auth_uuid}", &self.uuid);
			if let Some(xuid) = &self.xuid {
				*arg = arg.replace("${auth_xuid}", xuid);
			}
			if let Some(client_id) = &self.client_id {
				*arg = arg.replace("${clientid}", client_id);
			}
			if let Some(access_token) = &self.access_token {
				*arg = arg.replace("${auth_access_token}", access_token);
			}
		}

		debug!("Player set to: {}:{}", self.username, self.uuid);
	}
}

#[derive(Debug, Clone)]
pub struct LauncherInfo {
	pub name: String,
	pub version: String,
}

impl LauncherInfo {
	pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
		Self { name: name.into(), version: version.into() }
	}
}

impl Layer for LauncherInfo {
	fn apply(self, launch_mod: &mut LaunchMod) {
		for arg in &mut *launch_mod.java_args {
			*arg = arg.replace("${launcher_name}", &self.name);
			*arg = arg.replace("${launcher_version}", &self.version);
		}

		debug!("Launcher info set to: {} v{}", self.name, self.version);
	}
}

#[derive(Debug, Clone)]
pub struct WindowSize {
	pub width: u32,
	pub height: u32,
}

impl WindowSize {
	pub fn new(width: u32, height: u32) -> Self {
		Self { width, height }
	}
}

impl Layer for WindowSize {
	fn apply(self, launch_mod: &mut LaunchMod) {
		let width = self.width.to_string();
		let height = self.height.to_string();
		for arg in &mut *launch_mod.app_args {
			*arg = arg.replace("${resolution_width}", &width);
			*arg = arg.replace("${resolution_height}", &height);
		}
		debug!("Window size set to {width}x{height}");
	}
}

#[derive(Debug)]
pub struct Fullscreen;

impl Layer for Fullscreen {
	fn apply(self, launch_mod: &mut LaunchMod) {
		launch_mod.app_args.push("--fullscreen".into());
		debug!("Window size set to fullscreen");
	}
}
