mod builder;
mod digits;
mod prefix;
mod python;
mod secret;
mod suffix;

pub mod nric;

use crate::python::opennric::PyNRIC;
use crate::python::secretnric::PySecretNRIC;
use pyo3::prelude::*;

// A Python module implemented in Rust.
#[pymodule]
fn nric_do_not_use(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNRIC>()?;
    m.add_class::<PySecretNRIC>()?;
    Ok(())
}
