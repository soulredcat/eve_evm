// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{CryptoError, MAX_ML_DSA_MESSAGE_BYTES};

pub(crate) fn validate_mldsa65_input(message: &[u8], context: &[u8]) -> Result<(), CryptoError> {
    if message.len() > MAX_ML_DSA_MESSAGE_BYTES {
        return Err(CryptoError::MessageTooLarge);
    }
    if context.len() > 255 {
        return Err(CryptoError::ContextTooLarge);
    }
    Ok(())
}
