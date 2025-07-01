use pyo3::prelude::*;

/// Consume a Python `logging.LogRecord` and emit a Rust `tracing::Event` instead.
#[pyfunction]
fn host_log<'py>(record: Bound<'py, PyAny>) -> PyResult<()> {
	let level = record.getattr("levelno")?;
	let message = record.getattr("getMessage")?.call0()?;
	let pathname = record.getattr("pathname")?;
	let lineno = record.getattr("lineno")?;
	let logger_name = record.getattr("name")?;

	if level.ge(40u8)? {
		tracing::event!(tracing::Level::ERROR, %pathname, %lineno, %logger_name, "{message}");
	} else if level.ge(30u8)? {
		tracing::event!(tracing::Level::WARN, %pathname, %lineno, %logger_name, "{message}");
	} else if level.ge(20u8)? {
		tracing::event!(tracing::Level::INFO, %pathname, %lineno, %logger_name, "{message}");
	} else if level.ge(10u8)? {
		tracing::event!(tracing::Level::DEBUG, %pathname, %lineno, %logger_name, "{message}");
	} else {
		tracing::event!(tracing::Level::TRACE, %pathname, %lineno, %logger_name, "{message}");
	}

	Ok(())
}

/// Modifies the Python `logging` module to deliver its log messages to the host `tracing::Subscriber` by default.
/// To achieve this goal, the following changes are made to the module:
/// - A new builtin function `logging.host_log` transcodes `logging.LogRecord`s to `tracing::Event`s. This function
///   is not exported in `logging.__all__`, as it is not intended to be called directly.
/// - A new class `logging.HostHandler` provides a `logging.Handler` that delivers all records to `host_log`.
/// - `logging.basicConfig` is changed to use `logging.HostHandler` by default.
/// Since any call like `logging.warn(...)` sets up logging via `logging.basicConfig`, all log messages are now
/// delivered to `crate::host_log`, which will send them to `tracing::event!`.
pub fn setup_logging(py: Python) -> PyResult<()> {
	let logging = py.import("logging")?;

	logging.setattr("host_log", wrap_pyfunction!(host_log, &logging)?)?;

	py.run(
		cr#"
class HostHandler(Handler):
	def __init__(self, level=0):
		super().__init__(level=level)
	
	def emit(self, record):
		host_log(record)

oldBasicConfig = basicConfig
def basicConfig(*pargs, **kwargs):
	if "handlers" not in kwargs:
		kwargs["handlers"] = [HostHandler()]
	return oldBasicConfig(*pargs, **kwargs)
"#,
		Some(&logging.dict()),
		None,
	)?;

	let all = logging.index()?;
	all.append("HostHandler")?;

	Ok(())
}
