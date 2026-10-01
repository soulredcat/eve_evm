// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DurableSigner;

/// Runtime application/transport must fence signing on uncertain durable state.
pub(in crate::consensus) fn fence_signer(signer: &mut DurableSigner) {
    signer.fenced = true;
}
