// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Failures from bounded experimental signature verification and canonical binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CryptoError {
    InvalidSeedLength,
    InvalidPublicKeyLength,
    InvalidSignatureLength,
    InvalidSignatureEncoding,
    InvalidSignature,
    InvalidClassicalPublicKey,
    InvalidClassicalSignatureLength,
    InvalidClassicalSignature,
    MessageTooLarge,
    ContextTooLarge,
    WrongIdentity,
    WrongKeyEpoch,
    SigningFailed,
}
