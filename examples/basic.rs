use pyo3::prelude::*;

fn main() -> anyhow::Result<()> {
	// Prepare a subscriber for logging from the host
	tracing_subscriber::fmt::init();

	// The host can now use `tracing` as normal
	tracing::warn!("Rust put {} in the oven...", "pizza");

	// Ask pyo3 to set up embedded Python interpreter
	pyo3::Python::initialize();

	Python::attach(|py| -> anyhow::Result<()> {
		// Extend the `logging` module to interact with tracing
		tracing_for_pyo3_logging::setup_logging(py)?;

		// Python code can now `import logging` as usual
		py.run(c"import logging", None, None)?;

		// Log messages are forwarded to `tracing` and dealt with by the subscriber
		py.run(c"logging.error('Python let the %s burn!', 'pizza')", None, None)?;

		Ok(())
	})?;

	Ok(())
}
