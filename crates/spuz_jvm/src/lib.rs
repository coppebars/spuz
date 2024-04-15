use std::fmt::{Display, Formatter};
use std::{
	collections::HashMap,
	path::PathBuf,
	process::Stdio,
	sync::{atomic::AtomicBool, Arc},
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
	pub jargs: Vec<String>,
	pub aargs: Vec<String>,
	main_class: String,
	vars: HashMap<String, String>,
	agents: Vec<Agent>,

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
			agents: Vec::new(),
			stdx: broadcast::channel(64).0,
		}
	}

	pub fn var(&mut self, ident: impl Into<String>, value: impl Into<String>) {
		self.vars.insert(ident.into(), value.into());
	}

	pub fn var_opt(&mut self, ident: impl Into<String>, value: Option<impl Into<String>>) {
		if let Some(value) = value {
			self.var(ident, value);
		}
	}

	fn command(&self) -> Command {
		let mut cmd = Command::new(&self.bin);
		let jargs = set_vars(self.jargs.clone(), &self.vars);
		let aargs = set_vars(self.aargs.clone(), &self.vars);
		cmd
			.args(self.agents.iter().map(ToString::to_string))
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
			s.replace_range(.., &s.replace(&format!("${{{key}}}"), value));
		}
	}
	args
}

#[derive(Debug)]
pub struct Agent {
	pub path: PathBuf,
	pub options: Option<String>,
}

impl Agent {
	pub fn new(path: impl Into<PathBuf>) -> Self {
		let path = path.into();
		Self { path, options: None }
	}
}

impl Display for Agent {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let Self { path, options } = self;
		let path = path.to_str().ok_or(std::fmt::Error)?;

		write!(f, "-javaagent:{path}")?;

		if let Some(options) = options {
			write!(f, "={options}")?;
		}

		Ok(())
	}
}
