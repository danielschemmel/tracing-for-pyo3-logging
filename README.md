# tracing-for-pyo3-logging

Enables tracing for pyo3-based embedded Python applications using Python's `logging` module.

## Usage
Run `setup_logging` before using `logging` for the first time:
```rust
Python::attach(|py| {
	// Extend the `logging` module to interact with tracing
	tracing_for_pyo3_logging::setup_logging(py)
})?;
```

## Features
Enable the `log` feature if the host uses a `log` based logger instead.
