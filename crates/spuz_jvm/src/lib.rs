use std::{
	collections::HashMap,
	path::PathBuf,
	process::Stdio,
	sync::{Arc, atomic::AtomicBool},
};

use bytes::BytesMut;
use tokio::{
	io::AsyncReadExt,
	process::Command,
	sync::{broadcast, Mutex, Notify},
};

pub use err::{Error, Result};

mod err;

#[derive(Debug)]
pub struct Jvm {
	bin: PathBuf,
	jargs: Vec<String>,
	aargs: Vec<String>,
	main_class: String,
	vars: HashMap<String, String>,

	stdx: broadcast::Sender<String>,
}

impl Jvm {
	pub fn new(bin: PathBuf) -> Self {
		Self {
			bin,
			jargs: Vec::new(),
			aargs: Vec::new(),
			main_class: String::new(),
			vars: HashMap::new(),
			stdx: broadcast::channel(64).0,
		}
	}

	fn command(&self) -> Command {
		let mut cmd = Command::new(&self.bin);
		let jargs = set_vars(self.jargs.clone(), &self.vars);
		let aargs = set_vars(self.aargs.clone(), &self.vars);
		cmd
			.args(&jargs) // Jvm args
			.arg(&self.main_class) // Main class
			.args(&aargs) // App args (minecraft args)
			.stdout(Stdio::piped()) // Pipe stdout
			.stderr(Stdio::piped()); // Pipe stderr
		cmd
	}

	pub fn spawn(&self) -> Result<Process> {
		let mut child = self.command().spawn()?;
		let mut stdout = child.stdout.take().expect("Everything is broken");
		let mut stderr = child.stderr.take().expect("Everything is broken");

		let err = Arc::new(AtomicBool::new(false));
		let ended = Arc::new(Notify::new());

		tokio::spawn({
			let err = Arc::clone(&err);
			let ended = Arc::clone(&ended);
			let stdx = self.stdx.clone();

			async move {
				let mut stdout_buf = BytesMut::with_capacity(1024);
				let mut stderr_buf = BytesMut::with_capacity(1024);

				macro_rules! stdx_send {
					($stdx:ident, $ended:ident, $read:ident, $buf:ident) => {
						if let Ok(read) = $read {
							if read == 0 {
								$ended.notify_one();
								break;
							}

							let bytes = &$buf[..read];
							let Ok(str) = ::std::str::from_utf8(bytes) else { unreachable!() };
							if $stdx.send(str.to_owned()).is_err() {
								$ended.notify_one();
								break;
							}
						} else {
							err.store(true, ::std::sync::atomic::Ordering::Relaxed);
						}
					};
				}

				loop {
					tokio::select! {
						read = stdout.read_buf(&mut stdout_buf) => {
							stdx_send!(stdx, ended, read, stdout_buf);
						},
						read = stderr.read_buf(&mut stderr_buf) => {
							stdx_send!(stdx, ended, read, stderr_buf);
						}
					}
				}
			}
		});

		Ok(Process { err, ended, stdx: self.stdx.subscribe().into() })
	}
}

#[derive(Debug)]
pub struct Process {
	err: Arc<AtomicBool>,
	ended: Arc<Notify>,
	stdx: Mutex<broadcast::Receiver<String>>,
}

impl Process {
	pub async fn recv(&self) -> Option<String> {
		self.stdx.lock().await.recv().await.ok()
	}
}

fn set_vars(mut args: Vec<String>, vars: &HashMap<String, String>) -> Vec<String> {
	for s in &mut args {
		for (key, value) in vars {
			s.replace_range(.., &s.replace(key, value));
		}
	}
	args
}
