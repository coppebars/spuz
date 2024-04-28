use std::{collections::HashSet, ops::Deref, path::PathBuf};

use anyhow::Result;
use spuz_folder::Folder;
use spuz_jvm::{useful::AllocRange, CommandBuilder};
use spuz_launcher::{Fullscreen, LauncherWrench, Player, WindowSize};
use spuz_piston::Feature;

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
	telemetry::setup();

	let root = PathBuf::from("./minecraft");
	let folder = Folder::settle(&root).await?;
	let versions = folder.versions();
	let version_id = "1.20.4";
	let client_dir = root.join("versions").join(version_id);
	let version = versions.get("1.20.4");
	let manifest = version.manifest().await?.expect("Version not found");

	let mut builder = CommandBuilder::new("java");
	builder.apply(AllocRange(1024..4096));

	let wrench = LauncherWrench {
		manifest: manifest.deref().clone(),
		libraries_dir: root.join("libraries").into(),
		assets_dir: root.join("assets").into(),
		natives_dir: client_dir.join("natives").into(),
		client_jar: client_dir.join(format!("{version_id}.jar")).into(),
		game_dir: root.join("instances").join("test").into(),
		features: HashSet::from([Feature::CustomResolution]),
	};

	builder.apply(wrench);
	builder.apply(Player::new("LIMPIX31", "268903ca-7946-400a-8984-1fdc0b8baf71"));
	builder.apply(WindowSize::new(1280, 720));
	// builder.apply(Fullscreen);

	let mut process = builder.build().spawn()?;

	while let Some(log) = process.logs.recv().await {
		print!("{log}");
	}

	Ok(())
}
