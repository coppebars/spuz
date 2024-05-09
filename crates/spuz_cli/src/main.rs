use std::path::{Path, PathBuf};

use anyhow::Result;
use spuz_folder::Folder;
use spuz_spawner::{useful::AllocRange, CommandBuilder};
use spuz_wrench::{LauncherWrench, Player, WindowSize};

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
	telemetry::setup();

	let root = PathBuf::from("./minecraft");
	let folder = Folder::settle(&root).await?;
	let versions = folder.versions();
	let game_dir = Path::new("instances").join("test");
	let version = versions.get("1.20.4");
	#[allow(clippy::expect_used)]
	let manifest = version.manifest().await?.expect("Version not found");

	let mut builder = CommandBuilder::new("java");
	builder.apply(AllocRange(1024..4096));

	let wrench = LauncherWrench::builder().manifest(&manifest).current_dir(&root).game_dir(&game_dir).build();

	builder.apply(wrench);
	builder.apply(Player::new("LIMPIX31", "268903ca-7946-400a-8984-1fdc0b8baf71"));
	builder.apply(WindowSize::new(1280, 720));
	// builder.apply(Fullscreen);

	let process = builder.build().spawn()?;

	while let Ok(log) = process.logs.recv().await {
		print!("{log}");
	}

	Ok(())
}
