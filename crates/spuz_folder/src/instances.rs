use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub struct Instances {
	path: Arc<Path>
}

impl Instances {
	pub(crate) fn from_folder_root(root: &Path) -> Self {
		let path = root.join("instances").into();
		Self { path }
	}
}
