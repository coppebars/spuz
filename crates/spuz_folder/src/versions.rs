use std::{path::Path, str::FromStr, sync::Arc};

use spuz_piston::PistonPackage;
use tokio::{
	fs::{try_exists, File},
	io::AsyncReadExt,
	sync::OnceCell,
};

use crate::Result;

#[derive(Debug)]
pub struct Versions {
	path: Arc<Path>,
}

impl Versions {
	pub(crate) fn from_folder_root(root: &Path) -> Self {
		let path = root.join("versions").into();
		Self { path }
	}

	pub fn get(&self, id: &str) -> Version {
		Version::new(&self.path, id)
	}
}

type LazyManifest = OnceCell<Arc<PistonPackage>>;

#[derive(Debug)]
pub struct Version {
	pub id: String,
	pub path: Arc<Path>,
	pub manifest_path: Arc<Path>,
	pub client_path: Arc<Path>,
	pub natives_path: Arc<Path>,
	manifest: LazyManifest,
}

impl Version {
	pub(crate) fn new(versions_dir: &Path, id: &str) -> Self {
		Self {
			id: id.into(),
			path: versions_dir.join(id).into(),
			manifest_path: versions_dir.join(id).join(format!("{id}.json")).into(),
			client_path: versions_dir.join(id).join(format!("{id}.jar")).into(),
			natives_path: versions_dir.join(id).join("natives").into(),
			manifest: LazyManifest::new(),
		}
	}

	pub async fn manifest(&self) -> Result<Option<Arc<PistonPackage>>> {
		if try_exists(&self.path).await? {
			let initializer = || async move {
				// Prevent redundant allocations
				// Most manifestos are ~36Kb in size
				const COMMON_SIZE: usize = 0x9000;

				let mut file = File::open(&self.manifest_path).await?;
				let mut content = String::with_capacity(COMMON_SIZE);
				file.read_to_string(&mut content).await?;
				let manifest = PistonPackage::from_str(&content)?;
				Result::<_, crate::Error>::Ok(Arc::new(manifest))
			};

			let manifest = self.manifest.get_or_try_init(initializer).await?;

			Ok(Some(manifest.clone()))
		} else {
			Ok(None)
		}
	}
}
