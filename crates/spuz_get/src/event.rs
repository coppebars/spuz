#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
	JobStarted {
		tasks: usize,
		bytes: u64,
	},
	JobFinished,
	JobFailed,

	TaskStarted,
	TaskChunk {
		total: u64,
		size: usize,
	},
	TaskFinished,
	TaskFailed,
}
