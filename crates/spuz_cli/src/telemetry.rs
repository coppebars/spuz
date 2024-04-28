use tracing::Subscriber;
use tracing_subscriber as trsb;
use tracing_subscriber::{filter::LevelFilter, fmt::format::FmtSpan, EnvFilter, Layer};
use trsb::{layer::SubscriberExt, util::SubscriberInitExt};

fn new<S>() -> Box<dyn Layer<S> + Send + Sync>
where
	S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a> + Send + Sync,
{
	tracing_subscriber::fmt::layer()
		.compact()
		.with_ansi(true)
		.with_span_events(FmtSpan::NONE)
		.with_writer(std::io::stderr)
		.boxed()
}

pub(crate) fn setup() {
	let tracing_fmt = new();
	let directive = LevelFilter::TRACE.into();
	let tracing_env = EnvFilter::builder().with_default_directive(directive).from_env_lossy();
	let tracing_registry = trsb::registry().with(tracing_fmt).with(tracing_env);

	tracing_registry.init();
}
