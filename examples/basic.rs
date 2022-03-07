use pyo3::prelude::*;

fn main() -> anyhow::Result<()> {
	// Prepare a subscriber for logging from the host
	tracing_subscriber::fmt::init();

	// The host can now use `tracing` as normal
	tracing::warn!("Rust put pizza in the oven...");

	// Ask pyo3 to set up embedded Python interpreter
	pyo3::prepare_freethreaded_python();

	Python::with_gil(|py| -> anyhow::Result<()> {
		// Extend the `logging` module to interact with tracing
		tracing_for_pyo3_logging::setup_logging(py)?;

		// Python code can now `import logging` as usual
		py.run("import logging", None, None)?;

		// Log messages are forwarded to `tracing` and dealt with by the subscriber
		py.run("logging.error('Python let the pizza burn!')", None, None)?;

		Ok(())
	})?;

	Ok(())
}
