use anyhow::Result;

mod telemetry;

#[tokio::main]
async fn main() -> Result<()> {
	telemetry::setup();

	Ok(())
}
