use std::path::Path;
use std::sync::Arc;
use crate::Statefile;
use crate::statefile::StateWrite;
use crate::Result;

#[derive(Debug)]
pub struct FileDemand {
	root: Arc<Path>,
	statefile: Statefile
}

impl FileDemand {
	pub(crate) fn new(root: Arc<Path>, statefile: Statefile) -> Self {
		Self { root, statefile }
	}

	pub async fn write(&self) -> FdMod {
		let writer = self.statefile.write().await;
		FdMod { fa: self, writer }
	}
}

#[derive(Debug)]
pub struct FdMod<'a> {
	fa: &'a FileDemand,
	writer: StateWrite,
}

impl FdMod<'_> {
	pub fn increase(&mut self, path: &Path) {
		let acquisition_path = path.strip_prefix(&self.fa.root).unwrap_or(path);
		self.writer.increase_file_demand(acquisition_path.into());
	}

	pub fn decrease(&mut self, path: &Path) {
		let acquisition_path = path.strip_prefix(&self.fa.root).unwrap_or(path);
		self.writer.decrease_file_demand(acquisition_path);
	}

	pub async fn commit(&self) -> Result<()> {
		self.writer.save().await
	}
}
