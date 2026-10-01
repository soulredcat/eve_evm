// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{application::ConsensusApplication, approval::ApprovalRegistry};
use std::sync::Arc;

pub(in crate::consensus) fn application_approval_registry(
    application: &ConsensusApplication,
) -> Arc<ApprovalRegistry> {
    Arc::clone(&application.approvals)
}
