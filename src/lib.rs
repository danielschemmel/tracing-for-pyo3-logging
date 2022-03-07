use pyo3::prelude::*;

#[pyfunction]
fn host_log(record: &PyAny) -> PyResult<()> {
	let level = record.getattr("levelno")?;
	let message = record.getattr("getMessage")?.call0()?;

	if level.ge(40u8)? {
		tracing::event!(tracing::Level::ERROR, "{message}");
	} else if level.ge(30u8)? {
		tracing::event!(tracing::Level::WARN, "{message}");
	} else if level.ge(20u8)? {
		tracing::event!(tracing::Level::INFO, "{message}");
	} else if level.ge(10u8)? {
		tracing::event!(tracing::Level::DEBUG, "{message}");
	} else {
		tracing::event!(tracing::Level::TRACE, "{message}");
	}

	Ok(())
}

pub fn setup_logging(py: Python) -> PyResult<()> {
	let logging = py.import("logging")?;

	logging.setattr("host_log", wrap_pyfunction!(host_log, logging)?)?;

	py.run(
		r#"
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
		Some(logging.dict()),
		None,
	)?;

	let all = logging.index()?;
	all.append("HostHandler")?;

	Ok(())
}
