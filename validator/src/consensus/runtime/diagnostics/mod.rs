// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod classify_node_failure;
mod record_node_failure;
mod record_signer_request_position;
mod snapshot_node_failure;
mod types;
mod write_node_failure;
pub(in crate::consensus::runtime) use record_node_failure::record_node_failure;
pub(in crate::consensus::runtime) use record_signer_request_position::record_signer_request_position;
pub(in crate::consensus::runtime) use types::{NodeFailureRecord, SignerPosition};

#[cfg(test)]
pub(in crate::consensus::runtime) use classify_node_failure::classify_node_failure;
#[cfg(test)]
pub(in crate::consensus::runtime) use write_node_failure::write_node_failure;
