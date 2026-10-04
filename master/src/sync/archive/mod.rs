// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod open_proof_directory;
mod open_proof_file;
mod promote_staged_proof;
mod proof_file_name;
mod read_charged_proof;
mod reject_staged_proof;
mod scan_proof_archive;
mod sync_completed_proof;
mod sync_master_namespace;
mod write_staged_proof;

pub(super) use open_proof_directory::open_proof_directory;
pub(super) use promote_staged_proof::promote_staged_proof;
pub(super) use proof_file_name::proof_file_name;
pub(super) use read_charged_proof::read_charged_proof;
pub(super) use reject_staged_proof::reject_staged_proof;
pub(super) use scan_proof_archive::scan_proof_archive;
pub(super) use sync_completed_proof::sync_completed_proof;
pub(super) use sync_master_namespace::sync_master_namespace;
pub(super) use write_staged_proof::write_staged_proof;
pub(super) const STAGING: &str = "pending.proof";
pub(super) const REJECTED: &str = "rejected-staging.proof";
