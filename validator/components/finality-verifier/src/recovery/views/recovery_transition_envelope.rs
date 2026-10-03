// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery::{CompactRecoveryEnvelopeV1, VerifiedRecoveryTransition};

pub fn recovery_transition_envelope(
    transition: &VerifiedRecoveryTransition,
) -> &Arc<CompactRecoveryEnvelopeV1> {
    &transition.envelope
}
