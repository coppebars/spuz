use std::{path::Path, sync::Arc};

use url::Url;

#[derive(Debug, Clone)]
pub struct Task {
	pub url: Url,
	pub local: Arc<Path>,
	pub size: u64,

	pub lzma: bool,

	pub(crate) retries: u8,
}

impl Task {
	pub fn new(url: Url, local: Arc<Path>, size: u64) -> Self {
		Self {
			url,
			local,

			size,
			retries: 0,
			lzma: false,
		}
	}

	#[must_use]
	pub fn lzma(mut self, enable: bool) -> Self {
		self.lzma = enable;
		self
	}
	
	// TODO
	#[allow(unused)]
	pub(crate) fn retry_add(&mut self) {
		self.retries += 1;
	}
}
