// Authors: Joysusy & Violet Klaudia 💖

use pyo3::prelude::*;

mod crypto;
mod error;

#[pymodule]
fn lylacore(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crypto::Salt>()?;
    m.add_class::<crypto::Nonce>()?;
    m.add_class::<crypto::Key>()?;
    m.add_class::<crypto::AuthTag>()?;
    m.add_class::<crypto::DeriveKeyOptions>()?;

    m.add_function(wrap_pyfunction!(crypto::derive_key, m)?)?;
    m.add_function(wrap_pyfunction!(crypto::encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(crypto::decrypt, m)?)?;

    Ok(())
}
