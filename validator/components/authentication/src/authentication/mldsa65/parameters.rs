// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// FIPS 204 ML-DSA-65 public-key encoding length, in bytes.
pub const ML_DSA_65_PUBLIC_KEY_BYTES: usize = 1952;
/// FIPS 204 ML-DSA-65 signature encoding length, in bytes.
pub const ML_DSA_65_SIGNATURE_BYTES: usize = 3309;
/// Experimental payload bound; protocol-specific limits must additionally be enforced by callers.
pub const MAX_AUTHORIZATION_PAYLOAD_BYTES: usize = 128 * 1024;
/// Allows the bounded payload and the complete canonical enrollment/message envelope.
pub const MAX_ML_DSA_MESSAGE_BYTES: usize = MAX_AUTHORIZATION_PAYLOAD_BYTES + 4096;

pub type MlDsa65SigningKey = ml_dsa::SigningKey<ml_dsa::MlDsa65>;
