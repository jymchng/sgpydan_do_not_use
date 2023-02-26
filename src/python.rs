use crate::builder::{NRICBuilder, NoICDigits, NoICPrefix, NoICSuffix};
use crate::core::NRIC;
use crate::digits::ICDigits;
use crate::prefix::ICPrefixEnum;
use crate::suffix::ICSuffixEnum;
use pyo3::{
    class::iter::IterNextOutput,
    prelude::*,
    types::{PyDict, PyString, PyType},
    marker::Python,
    exceptions::{PyException, PyStopIteration},
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
  counter: u8
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
      Ok(PyNRICContainer {counter: 0})
  }
}

#[pymethods]
impl PyNRICContainer {

  fn __iter__(mut slf: PyRefMut<'_, Self>) -> Option<PyRefMut<'_, Self>> {
    if slf.counter == 0 {
          slf.counter = slf.counter + 1;
          Some(slf)
        } else {
          None
        }
  }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> IterNextOutput<PyRefMut<'_, Self>, &'static str> {
      if slf.counter == 0 {
          slf.counter = slf.counter + 1;
          IterNextOutput::Yield(slf)
        } else {
          IterNextOutput::Return("NRIC is no longer iterable.")
        }
    }

  #[classmethod]
  fn __call__(_cls: &PyType, v: &PyString) -> PyResult<PyNRIC> {
        let v: &str = v.extract()?;
        Ok(PyNRIC {
            rust_nric: NRIC::new(v).unwrap(),
        })
    }
}