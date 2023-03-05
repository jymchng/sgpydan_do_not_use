use crate::nric::NRIC;
use pyo3::exceptions::PyValueError;
use pyo3::intern;
use pyo3::types::PyTuple;
use pyo3::{
    prelude::*,
    types::{PyAny, PyType},
};
use std::fmt;

#[pyclass(name = "NRIC")]
#[derive(Debug, Clone)]
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
        let new_nric = NRIC::new(string);
        match new_nric {
            Ok(new_nric) => Ok(PyNRIC {
                rust_nric: new_nric,
            }),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[getter]
    pub fn get_prefix(&self) -> PyResult<String> {
        Ok(self.rust_nric.prefix.to_string())
    }

    #[getter]
    pub fn get_suffix(&self) -> PyResult<String> {
        Ok(self.rust_nric.suffix.to_string())
    }

    #[getter]
    pub fn get_digits(&self) -> PyResult<String> {
        Ok(self.rust_nric.digits.to_string())
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<NRIC::{}>", self.to_string()))
    }

    #[classmethod]
    pub fn __get_validators__(cls: &PyType) -> PyResult<&PyTuple> {
        let py = cls.py();
        let func = cls.getattr(intern!(py, "validate"))?;
        Ok(PyTuple::new(py, vec![func]))
    }

    #[classmethod]
    #[pyo3(text_signature = "(value)")]
    pub fn validate(_cls: &PyType, value: &PyAny) -> PyResult<PyNRIC> {
        let v: String = value.extract::<String>()?;
        PyNRIC::new(v)
    }
}
