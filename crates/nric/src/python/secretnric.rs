use crate::nric::NRIC;
use crate::secret::SecretNRICString;
use pyo3::exceptions::PyValueError;
use pyo3::intern;
use pyo3::types::PyTuple;
use pyo3::{
    prelude::*,
    types::{PyAny, PyType},
};

#[pyclass(name = "SecretNRIC")]
#[derive(Debug)]
pub struct PySecretNRIC {
    pub rust_secret_nric: SecretNRICString,
}

#[pymethods]
impl PySecretNRIC {
    #[new]
    pub fn new(string: String) -> PyResult<Self> {
        let new_nric = NRIC::new(string);
        match new_nric {
            Ok(new_nric) => Ok(PySecretNRIC {
                rust_secret_nric: SecretNRICString::new(new_nric),
            }),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok("<SECRETNRIC>".to_string())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok("<SECRETNRIC>".to_string())
    }

    pub fn encrypt(&self, filepath: &str, key: &str) -> anyhow::Result<String> {
        Ok(self.rust_secret_nric.encrypt(filepath, key)?)
    }

    #[staticmethod]
    pub fn decrypt(input: &str, filepath: &str, key: &str) -> anyhow::Result<String> {
        Ok(SecretNRICString::decrypt(input, filepath, key)?)
    }

    #[classmethod]
    pub fn __get_validators__(cls: &PyType) -> PyResult<&PyTuple> {
        let py = cls.py();
        let func = cls.getattr(intern!(py, "validate"))?;
        Ok(PyTuple::new(py, vec![func]))
    }

    #[classmethod]
    #[pyo3(text_signature = "(value)")]
    pub fn validate(_cls: &PyType, value: &PyAny) -> PyResult<Self> {
        let v: String = value.extract::<String>()?;
        Self::new(v)
    }
}
