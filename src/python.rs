use crate::builder::{NRICBuilder, NoICDigits, NoICPrefix, NoICSuffix};
use crate::core::NRIC;
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use pyo3::{prelude::*, types::PyType, PyTypeInfo};
use std::fmt;

#[pyclass(name = "NRIC")]
#[derive(Debug)]
pub struct PyNRIC {
    pub rust_nric: NRIC,
}

impl fmt::Display for PyNRIC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.rust_nric.prefix, self.rust_nric.digits, self.rust_nric.suffix
        )
    }
}

#[pymethods]
impl PyNRIC {
    #[new]
    pub fn new(string: String) -> PyResult<PyNRIC> {
        Ok(PyNRIC {
            rust_nric: NRIC::new(string).unwrap(),
        })
    }

    #[getter]
    fn get_prefix(&self) -> PyResult<String> {
        Ok(self.rust_nric.prefix.to_string())
    }

    #[getter]
    fn get_suffix(&self) -> PyResult<String> {
        Ok(self.rust_nric.suffix.to_string())
    }

    #[getter]
    fn get_digits(&self) -> PyResult<String> {
        Ok(self.rust_nric.digits.to_string())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<NRIC::{}>", self.to_string()))
    }

    #[classmethod]
    fn __get_validators__(_: &PyType) -> PyResult<()> {
      Ok(())
    }
}
