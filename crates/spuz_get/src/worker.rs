use std::{
	io,
	io::ErrorKind,
	pin::Pin,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	task::{Context, Poll},
};

use async_compression::futures::bufread::LzmaDecoder;
use futures::{io::BufReader, AsyncBufRead, AsyncRead, AsyncReadExt, TryStreamExt};
use pin_project::pin_project;
use reqwest::Client;
use tokio::{
	fs::File,
	io::AsyncWriteExt,
	sync::{mpsc, Semaphore},
};

use crate::{job::JobHandle, loop_select, result_async, spawn, Event, Job};

#[derive(Debug)]
pub struct Worker {
	client: Client,
	semaphore: Arc<Semaphore>,
}

impl Worker {
	pub fn new(client: Client, concurrency: usize) -> Self {
		let semaphore = Arc::new(Semaphore::new(concurrency));

		Self { client, semaphore }
	}

	pub fn push(&self, job: Job) -> Arc<JobHandle> {
		let (tx, rx) = mpsc::unbounded_channel();
		let handle = Arc::new(JobHandle::new(rx));
		let counter = Arc::new(AtomicUsize::new(job.total));

		tx.send(Event::JobStarted {
			bytes: job.size,
			tasks: job.total,
		})
		.unwrap();

		for task in job.tasks {
			let counter = counter.clone();
			let tx = tx.clone();
			let tx2 = tx.clone();
			let handle = handle.clone();
			let client = self.client.clone();
			let semaphore = self.semaphore.clone();

			spawn! {
				let permit = semaphore.acquire().await.expect("Semaphore unexpectedly closed");

				tx.send(Event::TaskStarted).unwrap();

				let result = result_async! {
					let request = client.get(task.url);
					let response = request.send().await?;

					let stream = response.bytes_stream().map_err(|err| io::Error::new(ErrorKind::Other, err));
					let reader = stream.into_async_read();
					let reader = TrackingReader::new(reader, &tx, task.size);

					let mut file = File::create(task.local).await?;

					let piped = task.lzma;

					let mut decoder: Box<dyn AsyncRead + Send + Sync + Unpin> = if task.lzma {
						Box::new(LzmaDecoder::new(reader))
					} else {
						Box::new(reader)
					};

					let mut buf = vec![0; 16384];

					loop_select! {
						() = handle.ct.cancelled() => {
							break;
						}
						read = decoder.read(&mut buf) => {
							let read = read?;

							if read == 0 {
								tx.send(Event::TaskFinished).unwrap();
								if counter.fetch_sub(1, Ordering::Relaxed) == 1 {
									tx.send(Event::JobFinished).unwrap();
								};
								return Ok(());
							}

							if !piped {
								tx.send(Event::TaskChunk {
									total: task.size,
									size: read,
								}).unwrap();
							}

							file.write_all(&buf[..read]).await?;
						}
					}

					Ok(())
				};

				if let Err(_err) = result.await {
					tx2.send(Event::TaskFailed).unwrap();
				}

				drop(permit)
			};
		}

		handle
	}
}

#[pin_project]
pub struct TrackingReader<'a, T> {
	#[pin]
	inner: BufReader<T>,
	total: u64,
	tx: &'a mpsc::UnboundedSender<Event>,
}

impl<'a, T> TrackingReader<'a, T>
where
	T: AsyncRead,
{
	pub fn new(reader: T, tx: &'a mpsc::UnboundedSender<Event>, total: u64) -> Self {
		Self {
			inner: BufReader::new(reader),
			total,
			tx,
		}
	}
}

impl<T> AsyncRead for TrackingReader<'_, T>
where
	T: Unpin + AsyncRead,
{
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
		let this = self.project();
		this.inner.poll_read(cx, buf)
	}
}

impl<T> AsyncBufRead for TrackingReader<'_, T>
where
	T: Unpin + AsyncRead,
{
	fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
		let this = self.project();
		let result = this.inner.poll_fill_buf(cx);
		if let Poll::Ready(Ok(result)) = &result {
			let event = Event::TaskChunk {
				total: *this.total,
				size: result.len(),
			};
			this.tx.send(event).unwrap();
		}
		result
	}

	fn consume(self: Pin<&mut Self>, amt: usize) {
		let this = self.project();
		this.inner.consume(amt);
	}
}
