// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{DurableSigner, SignerStatus};

pub(in crate::consensus) fn signer_status(signer: &DurableSigner) -> SignerStatus {
    SignerStatus {
        last_height: signer.last.as_ref().map(|record| record.hrs.height),
        last_round: signer.last.as_ref().map(|record| record.hrs.round),
        last_step: signer.last.as_ref().map(|record| record.hrs.step),
        cursor: signer.cursor,
        fenced: signer.fenced,
    }
}
