mod codec;
mod key_generation;
mod parameters;
mod signing;
mod validation;
mod verification;

pub use codec::export_mldsa65_public_key;
pub use key_generation::derive_mldsa65_key;
pub use parameters::{
    MAX_AUTHORIZATION_PAYLOAD_BYTES, MAX_ML_DSA_MESSAGE_BYTES, ML_DSA_65_PUBLIC_KEY_BYTES,
    ML_DSA_65_SIGNATURE_BYTES, MlDsa65SigningKey,
};
pub use signing::sign_mldsa65;
pub use verification::verify_mldsa65;
