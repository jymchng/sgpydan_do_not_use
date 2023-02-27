mod builder;
mod digits;
mod prefix;
mod python;
mod suffix;

pub mod core;

use crate::python::PyNRIC;
use pyo3::prelude::*;

// A Python module implemented in Rust.
#[pymodule]
fn sgnric_do_not_use(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNRIC>()?;
    Ok(())
}
