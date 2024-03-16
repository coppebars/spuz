use std::path::PathBuf;

use spuz_spec::internals::{Url, VersionId};

#[derive(Debug, Copy, Clone)]
pub enum Kind {
	ClientManifest,
}

#[derive(Debug, Clone)]
pub enum Plan {
	Download { url: Url, kind: Kind, path: PathBuf },
}

#[derive(Debug, Clone)]
pub struct Ir {
	pub id: VersionId,
}
