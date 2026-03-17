// Authors: Joysusy & Violet Klaudia 💖

#![allow(clippy::useless_conversion)]

use crate::error::crypto_error_to_py;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use soul_crypto::KeyDerivationAlgorithm;

#[pyclass]
#[derive(Clone)]
pub struct Salt {
    inner: soul_crypto::Salt,
}

#[pymethods]
impl Salt {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        let inner = soul_crypto::Salt::from_slice(data).map_err(crypto_error_to_py)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    fn generate() -> Self {
        Self {
            inner: soul_crypto::generate_salt(),
        }
    }

    fn as_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.inner.as_bytes())
    }

    fn __repr__(&self) -> String {
        format!("Salt({})", hex::encode(self.inner.as_bytes()))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Nonce {
    inner: soul_crypto::Nonce,
}

#[pymethods]
impl Nonce {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        let inner = soul_crypto::Nonce::from_slice(data).map_err(crypto_error_to_py)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    fn generate() -> Self {
        Self {
            inner: soul_crypto::generate_nonce(),
        }
    }

    fn as_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.inner.as_bytes())
    }

    fn __repr__(&self) -> String {
        format!("Nonce({})", hex::encode(self.inner.as_bytes()))
    }
}

#[pyclass]
pub struct Key {
    inner: soul_crypto::Key,
}

#[pymethods]
impl Key {
    fn as_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.inner.as_bytes())
    }

    fn __repr__(&self) -> String {
        "Key([REDACTED])".to_string()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct AuthTag {
    inner: soul_crypto::AuthTag,
}

#[pymethods]
impl AuthTag {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        let inner = soul_crypto::AuthTag::from_slice(data).map_err(crypto_error_to_py)?;
        Ok(Self { inner })
    }

    fn as_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.inner.as_bytes())
    }

    fn __repr__(&self) -> String {
        format!("AuthTag({})", hex::encode(self.inner.as_bytes()))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct DeriveKeyOptions {
    #[pyo3(get, set)]
    pub algorithm: String,
}

#[pymethods]
impl DeriveKeyOptions {
    #[new]
    #[pyo3(signature = (algorithm = "argon2id".to_string()))]
    fn new(algorithm: String) -> Self {
        Self { algorithm }
    }
}

#[pyfunction]
#[pyo3(signature = (passphrase, salt, options=None))]
pub fn derive_key<'py>(
    py: Python<'py>,
    passphrase: String,
    salt: &Salt,
    options: Option<DeriveKeyOptions>,
) -> PyResult<Bound<'py, PyAny>> {
    let salt_inner = salt.inner.clone();
    let algorithm = options
        .as_ref()
        .map(|o| match o.algorithm.as_str() {
            "argon2id" => KeyDerivationAlgorithm::Argon2id,
            "pbkdf2" => KeyDerivationAlgorithm::Pbkdf2,
            "scrypt" => KeyDerivationAlgorithm::Scrypt,
            _ => KeyDerivationAlgorithm::Argon2id,
        })
        .unwrap_or(KeyDerivationAlgorithm::Argon2id);

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let key = soul_crypto::derive_key(&passphrase, &salt_inner, algorithm)
            .await
            .map_err(crypto_error_to_py)?;
        Ok(Key { inner: key })
    })
}

#[pyfunction]
pub fn encrypt<'py>(
    py: Python<'py>,
    plaintext: &[u8],
    key: &Key,
) -> PyResult<(Nonce, Bound<'py, PyBytes>, AuthTag)> {
    let result = py.allow_threads(|| soul_crypto::encrypt(&key.inner, plaintext));

    let (nonce_inner, ciphertext, auth_tag_inner) = result.map_err(crypto_error_to_py)?;
    let ciphertext_bytes = PyBytes::new_bound(py, &ciphertext);

    Ok((
        Nonce { inner: nonce_inner },
        ciphertext_bytes,
        AuthTag {
            inner: auth_tag_inner,
        },
    ))
}

#[pyfunction]
pub fn decrypt<'py>(
    py: Python<'py>,
    ciphertext: &[u8],
    key: &Key,
    nonce: &Nonce,
    auth_tag: &AuthTag,
) -> PyResult<Bound<'py, PyBytes>> {
    let result = py.allow_threads(|| {
        soul_crypto::decrypt(&key.inner, &nonce.inner, ciphertext, &auth_tag.inner)
    });

    let plaintext = result.map_err(crypto_error_to_py)?;
    Ok(PyBytes::new_bound(py, &plaintext))
}
