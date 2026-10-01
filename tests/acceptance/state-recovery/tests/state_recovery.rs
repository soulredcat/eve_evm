// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "cases/canonical_payload.rs"]
mod canonical_payload;
#[path = "cases/durable_identity.rs"]
mod durable_identity;
#[path = "cases/execution_identity.rs"]
mod execution_identity;
#[path = "cases/genesis_state.rs"]
mod genesis_state;
#[path = "cases/independent_roots.rs"]
mod independent_roots;
#[path = "cases/journal_views.rs"]
mod journal_views;
#[path = "cases/logical_integrity.rs"]
mod logical_integrity;
#[path = "cases/master_authority.rs"]
mod master_authority;
#[path = "cases/process_atomicity.rs"]
mod process_atomicity;
#[path = "cases/snapshot_interruption.rs"]
mod snapshot_interruption;
#[path = "cases/snapshot_transfer.rs"]
mod snapshot_transfer;
#[path = "cases/storage_corruption.rs"]
mod storage_corruption;
#[path = "support/mod.rs"]
mod support;
#[path = "cases/validated_views.rs"]
mod validated_views;
