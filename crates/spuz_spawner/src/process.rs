use async_channel::Receiver;
use tokio::process::Command;
#[cfg(feature = "process-handle")]
use {
	crate::Result,
	async_channel::unbounded,
	std::sync::Arc,
	tokio::{io::AsyncReadExt, sync::Notify},
};

#[derive(Debug)]
pub struct LaunchCommand {
	command: Command,
}

impl LaunchCommand {
	pub(crate) fn new(command: Command) -> Self {
		Self { command }
	}

	pub fn into_command(self) -> Command {
		self.command
	}

	#[cfg(feature = "process-handle")]
	pub fn spawn(self) -> Result<ProcessHandle> {
		let mut cmd = self.into_command();
		let mut child = cmd.spawn()?;
		// # SAFETY
		// Invariant: The instance has been created here and stdout, stderr have not
		// been moved until now
		#[allow(clippy::unwrap_used)]
		let mut stdout = child.stdout.take().unwrap();
		#[allow(clippy::unwrap_used)]
		let mut stderr = child.stderr.take().unwrap();

		let exit = Arc::new(Notify::new());
		let (logs_tx, logs) = unbounded();

		let handle = ProcessHandle { exit: exit.clone(), logs };

		tokio::spawn(async move {
			let mut stdout_buf = vec![0u8; 2048];
			let mut stderr_buf = vec![0u8; 2048];

			macro_rules! stdx_send {
				($logs_tx:ident, $exit:ident, $read:ident, $buf:ident) => {
					match $read {
						Ok(read) => {
							if read == 0 {
								$exit.notify_one();
								break;
							}

							let bytes = &$buf[..read];
							let str = ::std::str::from_utf8(bytes).unwrap();
							if $logs_tx.send(str.to_owned()).await.is_err() {
								$exit.notify_one();
								break;
							}
						}
						Err(err) => {
							::tracing::error!(?err, "An esrror occurred while reading stdout/stderr");
							$exit.notify_one();
						}
					}
				};
			}

			loop {
				tokio::select! {
					read = stdout.read(&mut stdout_buf) => {
						stdx_send!(logs_tx, exit, read, stdout_buf);
					},
					read = stderr.read(&mut stderr_buf) => {
						stdx_send!(logs_tx, exit, read, stderr_buf);
					}
				}
			}
		});

		Ok(handle)
	}
}

impl From<LaunchCommand> for Command {
	fn from(value: LaunchCommand) -> Self {
		value.into_command()
	}
}

#[cfg(feature = "process-handle")]
#[derive(Debug)]
pub struct ProcessHandle {
	pub exit: Arc<Notify>,
	pub logs: Receiver<String>,
}
