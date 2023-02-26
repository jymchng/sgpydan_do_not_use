use crate::builder::{NRICBuilder, NoICDigits, NoICPrefix, NoICSuffix};
use crate::core::NRIC;
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use pyo3::{
    class::iter::IterNextOutput,
    exceptions::{PyException, PyStopIteration},
    marker::Python,
    prelude::*,
    types::{PyDict, PyString, PyType, PyAny},
};
use std::fmt;
use std::vec::IntoIter;

#[pyclass(name = "NRIC")]
#[derive(Debug, Clone)]
pub struct PyNRIC {
    pub rust_nric: NRIC,
}

#[pyclass(name = "PyNRICContainer")]
#[derive(Debug)]
pub struct PyNRICContainer {
    #[pyo3(get, set)]
    boolean: bool,
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
    fn new(string: String) -> PyResult<PyNRIC> {
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

    // fn __repr__(&self) -> PyResult<String> {
    //     Ok(format!("<NRIC::{}>", self.to_string()))
    // }

    #[classmethod]
    fn __get_validators__(_cls: &PyType) -> PyResult<PyNRICContainer> {
        Ok(PyNRICContainer {boolean: true})
    }
}

#[pymethods]
impl PyNRICContainer {

    // #[new]
    // fn new() -> PyResult<PyNRICContainer> {
    //     Ok(PyNRICContainer { boolean: true })
    // }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> IterNextOutput<PyRefMut<'_, Self>, &'static str> {
        if slf.boolean {
            slf.boolean = false;
            IterNextOutput::Yield(slf)
        } else {
            IterNextOutput::Return("No Longer Iterable.")
        }
    }

    #[classmethod]
    fn __call__(cls: &PyType, v: &PyString) -> PyResult<PyNRIC> {
        let v: String = v.extract()?;
        PyNRIC::new(v)
    }

    // #[pyo3(signature = "cls", "v", "values")]
    // fn __call__(&mut self, py: Python<'_>, value: &PyString, values: Option<&PyDict>) -> PyResult<PyNRIC> {
    //     let v: String = value.extract()?;
    //     PyNRIC::new(v)
    // }
    
}
