// Authors: Joysusy & Violet Klaudia 💖

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use soul_crypto::CryptoError;

#[inline]
pub fn crypto_error_to_py(err: CryptoError) -> PyErr {
    PyValueError::new_err(err.to_string())
}
