use std::{collections::HashSet, ops::Deref};

use anyhow::Result;
use spuz_folder::Folder;
use spuz_launcher::Launcher;
use tokio::fs;

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
	telemetry::setup();

	let root = fs::canonicalize("./minecraft").await?;
	let folder = Folder::settle(&root).await?;
	let versions = folder.versions();
	let version = versions.get("1.20.4");

	let manifest = version.manifest().await?.expect("No manifest found");

	let launcher = Launcher::builder()
		.bin("java".into())
		.package(manifest.deref().clone().into())
		.player_name("LIMPIX31".into())
		.assets_dir(root.join("assets"))
		.libraries_dir(root.join("libraries"))
		.natives_dir(version.natives_path.deref().to_owned())
		.game_dir(root.join("instances").join("test"))
		.uuid(Some("a5090e59-0ff3-48f8-8135-39785da3e501".into()))
		.access_token(None)
		.resolution(None)
		.xuid(None)
		.launcher_name(None)
		.launcher_version(None)
		.client_id(None)
		.features(HashSet::new())
		.client_jar(root.join("versions").join("1.20.4").join("1.20.4.jar"))
		.build();

	let jvm = launcher.compile();

	let process = jvm.spawn()?;

	while let Some(msg) = process.recv().await {
		print!("{msg}");
	}

	Ok(())
}
