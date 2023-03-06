use crate::nric::NRIC;
use crate::secret::SecretNRICString;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "SecretNRIC")]
#[pyo3(text_signature="(string, filepath, key_var)")]
#[derive(Debug)]
pub struct PySecretNRIC {
    pub rust_secret_nric: SecretNRICString,
}

#[pymethods]
impl PySecretNRIC {
    #[new]
    pub fn new(string: String, filepath: &str, key_var: &str) -> PyResult<Self> {
        let new_nric = NRIC::new(string);
        match new_nric {
            Ok(new_nric) => Ok(PySecretNRIC {
                rust_secret_nric: SecretNRICString::new(new_nric, filepath, key_var)?,
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

    pub fn reveal_encrypted(&self) -> PyResult<String> {
        Ok(self.rust_secret_nric.encrypted_nric.clone())
    }

    #[staticmethod]
    pub fn decrypt(input: &str, filepath: &str, key: &str) -> anyhow::Result<String> {
        Ok(SecretNRICString::decrypt(input, filepath, key)?)
    }
}
