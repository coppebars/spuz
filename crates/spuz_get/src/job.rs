use std::{path::Path, sync::Arc};

use tokio::sync::{mpsc, Mutex, OwnedMutexGuard};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::{Event, Task};

#[derive(Debug, Clone)]
pub struct Job {
	pub tasks: Vec<Task>,
	pub total: usize,
	pub size: u64,
}

impl Job {
	pub fn new(tasks: Vec<Task>) -> Self {
		let total = tasks.len();
		let size = tasks.iter().map(|it| it.size).reduce(|a, b| a + b).unwrap_or_default();

		Self { tasks, total, size }
	}
}

#[derive(Debug)]
pub struct JobHandle {
	pub(crate) ct: CancellationToken,
	rx: Arc<Mutex<mpsc::UnboundedReceiver<Event>>>,
}

impl JobHandle {
	pub fn new(rx: mpsc::UnboundedReceiver<Event>) -> Self {
		let ct = CancellationToken::new();
		let rx = Arc::new(Mutex::new(rx));

		Self { ct, rx }
	}

	pub async fn rx(&self) -> OwnedMutexGuard<mpsc::UnboundedReceiver<Event>> {
		self.rx.clone().lock_owned().await
	}

	pub fn cancel(&self) {
		self.ct.cancel();
	}
}

#[derive(Debug, Default, Clone)]
pub struct JobBuilder {
	tasks: Vec<Task>,
}

impl JobBuilder {
	pub fn push(&mut self, url: Url, local: &Arc<Path>, size: u64) {
		self.tasks.push(Task::new(url, local.clone(), size));
	}

	pub fn into_job(self) -> Job {
		Job::new(self.tasks)
	}
}

impl From<JobBuilder> for Job {
	fn from(value: JobBuilder) -> Self {
		value.into_job()
	}
}
