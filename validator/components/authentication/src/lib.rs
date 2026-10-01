// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Experimental authentication primitives; this crate does not implement consensus or enrollment.
#![forbid(unsafe_code)]

mod authentication;

pub use authentication::errors::CryptoError;
pub use authentication::hybrid::{
    AuthorizationPurpose, EnrolledHybridIdentity, HybridAuthorization, HybridSignature,
    encode_hybrid_message, verify_hybrid_authorization,
};
pub use authentication::mldsa65::{
    MAX_AUTHORIZATION_PAYLOAD_BYTES, MAX_ML_DSA_MESSAGE_BYTES, ML_DSA_65_PUBLIC_KEY_BYTES,
    ML_DSA_65_SIGNATURE_BYTES, MlDsa65SigningKey, derive_mldsa65_key, export_mldsa65_public_key,
    sign_mldsa65, verify_mldsa65,
};
