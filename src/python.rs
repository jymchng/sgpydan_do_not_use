use crate::builder::{NRICBuilder, NoICDigits, NoICPrefix, NoICSuffix};
use crate::core::NRIC;
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use pyo3::exceptions::{PyValueError, PyAttributeError};
use pyo3::intern;
use pyo3::types::PyTuple;
use pyo3::{
    class::iter::IterNextOutput,
    exceptions::{PyException, PyStopIteration},
    marker::Python,
    prelude::*,
    types::{PyDict, PyString, PyType, PyAny, PyIterator, PyFunction, PyCFunction},
};
use std::fmt;
use std::vec::IntoIter;


#[pyclass(name = "NRIC")]
#[derive(Debug, Clone)]
pub struct PyNRIC {
    pub rust_nric: NRIC,
    boolean: bool,
}

#[pyclass(name = "PyNRICContainer")]
#[derive(Debug)]
#[pyo3(text_signature = "(cls, value, values, config, field)")]
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
    pub fn new(string: String) -> PyResult<PyNRIC> {
        let new_nric = NRIC::new(string);
        match new_nric {
            Ok(new_nric) => Ok(PyNRIC {
                rust_nric: new_nric,
                boolean: true
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


    // pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
    //     slf
    // }

    #[classmethod]
    pub fn __get_validators__(cls: &PyType) -> PyResult<&PyTuple> {
       let py = cls.py();
       let func = cls.getattr(intern!(py, "validate"))?;
       Ok(PyTuple::new(py, vec![func]))
    }

    // so the trick is to implement `__next__` for PyNRIC and returning the Yield(function)
    // then also implement `__iter__`; only `__next__` can return IterNextOutput enum
    // pub fn __next__(slf: &PyCell<Self>) -> IterNextOutput<&PyAny, &'static str> {
    //     let func = slf.get_type().getattr(intern!(slf.py(), "validate"));
    //     match func {
    //         Ok(func) => IterNextOutput::Yield(func),
    //         Err(_err) => IterNextOutput::Return("No longer iterable.")
    //     }
    // }

    #[classmethod]
    #[pyo3(text_signature="(value)")]
    pub fn validate(cls: &PyType, value: &PyAny) -> PyResult<PyNRIC> {
        let v: String = value.extract::<String>()?;
        PyNRIC::new(v)
    }

    // fn __call__(&self) -> PyResult<&str> {
    //     Ok("Hello, World!")
    // }
}

#[pymethods]
impl PyNRICContainer {

    #[new]
    pub fn new() -> PyResult<PyNRICContainer> {
        Ok(PyNRICContainer { boolean: true })
    }

    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<'_, Self>) -> IterNextOutput<PyRefMut<'_, Self>, &'static str> {
        if slf.boolean {
            slf.boolean = false;
            IterNextOutput::Yield(slf)
        } else {
            IterNextOutput::Return("No Longer Iterable.")
        }
    }

    #[staticmethod]
    pub fn __call__(value: &PyString, values: &PyDict, config: &PyAny, field: &PyAny) -> PyResult<PyNRIC> {
        let v: String = value.extract()?;
        PyNRIC::new(v)
    }
    
}


#[pyfunction]
#[pyo3(text_signature = "(cls, value, values, config, field)")]
pub fn validate(cls: &PyType, value: &PyString) -> PyResult<PyNRIC> {
    let v: String = value.extract()?;
    PyNRIC::new(v)
}


