use std::{error::Error, fs, io::Read, path::PathBuf};

use bpaf::Bpaf;
use serde_json::from_str;
use spuz_spec::vanilla::client::latest::ClientVersion;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
pub struct Cli {
	#[bpaf(positional("VERSION"), help("Path to version.json to launch from"))]
	pub version: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
	let options = cli().fallback_to_usage().run();
	let mut version_file = fs::File::open(options.version)?;
	let mut version_content = String::new();
	version_file.read_to_string(&mut version_content)?;
	let client: ClientVersion = from_str(&version_content)?;

	println!("{client:#?}");

	Ok(())
}
