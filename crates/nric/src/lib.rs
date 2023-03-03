mod builder;
mod digits;
mod prefix;
mod python;
mod suffix;

pub mod core;

use crate::python::opennric::PyNRIC;
use crate::python::secretnric::SecretNRIC;
use pyo3::prelude::*;

// A Python module implemented in Rust.
#[pymodule]
fn nric_do_not_use(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNRIC>()?;
    m.add_class::<SecretNRIC>()?;
    Ok(())
}
