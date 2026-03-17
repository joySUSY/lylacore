// Authors: Joysusy & Violet Klaudia 💖

use napi::bindgen_prelude::*;

pub fn crypto_error_to_napi(err: soul_crypto::CryptoError) -> Error {
    Error::from_reason(err.to_string())
}

#[allow(dead_code)]
pub fn coach_error_to_napi(err: coach_engine::CoachError) -> Error {
    Error::from_reason(err.to_string())
}
