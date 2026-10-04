// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use tokio::sync::OwnedSemaphorePermit;
pub(crate) struct RpcLeases {
    pub active: OwnedSemaphorePermit,
    pub worker: OwnedSemaphorePermit,
    pub bytes: OwnedSemaphorePermit,
    pub signature: Option<OwnedSemaphorePermit>,
    pub applied_history: Option<std::sync::Arc<crate::sync::applied::AppliedPublication>>,
}
pub(crate) struct SignatureLeases {
    pub active: OwnedSemaphorePermit,
    pub signature: OwnedSemaphorePermit,
    pub bytes: OwnedSemaphorePermit,
}
