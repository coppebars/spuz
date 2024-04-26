use anyhow::Result;
use spuz_folder::Folder;
use tokio::fs;

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
	telemetry::setup();

	let root = fs::canonicalize("./minecraft").await?;
	let folder = Folder::settle(&root).await?;
	// let versions = folder.versions();
	// let version = versions.get("1.20.4");
	//
	// let manifest = version.manifest().await?.expect("No manifest found");
	//
	// println!("{manifest:#?}");

	let jres = folder.jres();
	let jre = jres.get("java-runtime-delta");

	println!("{:?}", jre.bin());

	Ok(())
}
