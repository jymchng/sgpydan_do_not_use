#![allow(unused_imports)]
#![allow(dead_code)]

mod builder;
mod digits;
mod prefix;
mod suffix;
mod python;

pub mod core;

use crate::core::NRIC;
use crate::python::{PyNRIC, PyNRICContainer};
use pyo3::prelude::*;

// A Python module implemented in Rust.
#[pymodule]
fn sgnric_do_not_use(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNRIC>()?;
    m.add_class::<PyNRICContainer>()?;
    Ok(())
}
