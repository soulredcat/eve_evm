// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{application::ConsensusApplication, signing::fence_signer};

pub(in crate::consensus::application) fn fence_application(application: &mut ConsensusApplication) {
    application.fenced = true;
    let mut signer = application
        .signer
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    fence_signer(&mut signer);
}
