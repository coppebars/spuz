use std::{path::Path, str::FromStr, sync::Arc};

use spuz_piston::PistonManifest;
use tokio::{
	fs::{try_exists, File},
	io::AsyncReadExt,
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
		Version::new(self.path.clone(), id)
	}
}

type LazyManifest = tokio::sync::OnceCell<Arc<PistonManifest>>;
type LazyPath = std::cell::OnceCell<Arc<Path>>;

#[derive(Debug)]
pub struct Version {
	pub id: String,
	versions_dir: Arc<Path>,
	path: LazyPath,
	manifest_path: LazyPath,
	client_path: LazyPath,
	natives_path: LazyPath,
	manifest: LazyManifest,
}

impl Version {
	pub(crate) fn new(versions_dir: Arc<Path>, id: &str) -> Self {
		Self {
			id: id.into(),
			versions_dir,
			path: LazyPath::new(),
			manifest_path: LazyPath::new(),
			client_path: LazyPath::new(),
			natives_path: LazyPath::new(),
			manifest: LazyManifest::new(),
		}
	}

	pub fn path(&self) -> &Arc<Path> {
		self.path.get_or_init(|| self.versions_dir.join(&self.id).into())
	}

	pub fn manifest_path(&self) -> &Arc<Path> {
		self.manifest_path.get_or_init(|| self.path().join(format!("{}.json", self.id)).into())
	}

	pub fn client_path(&self) -> &Arc<Path> {
		self.client_path.get_or_init(|| self.path().join(format!("{}.json", self.id)).into())
	}

	pub fn natives_path(&self) -> &Arc<Path> {
		self.natives_path.get_or_init(|| self.path().join("natives").into())
	}

	pub async fn manifest(&self) -> Result<Option<Arc<PistonManifest>>> {
		if try_exists(self.path()).await? {
			let initializer = || async move {
				// Prevent redundant allocations
				// Most manifestos are ~36Kb in size
				const COMMON_SIZE: usize = 0x9000;

				let mut file = File::open(self.manifest_path()).await?;
				let mut content = String::with_capacity(COMMON_SIZE);
				file.read_to_string(&mut content).await?;
				let manifest = PistonManifest::from_str(&content)?;
				Result::<_, crate::Error>::Ok(Arc::new(manifest))
			};

			let manifest = self.manifest.get_or_try_init(initializer).await?;

			Ok(Some(manifest.clone()))
		} else {
			Ok(None)
		}
	}

	pub async fn exists(&self) -> Result<bool> {
		try_exists(self.manifest_path()).await.map_err(Into::into)
	}
}
