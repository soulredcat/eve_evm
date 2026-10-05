// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedMode;
use eve_storage::records::OpaqueRecordIdentity;
use sha2::{Digest, Sha256};

/// Local namespace mode binding, with no consensus/finality or enrollment authority.
pub(super) fn derive_effective_storage_identity(
    mut identity: OpaqueRecordIdentity,
    mode: AppliedMode,
) -> OpaqueRecordIdentity {
    if mode == AppliedMode::AuthenticatedImport {
        let mut hash = Sha256::new();
        hash.update(b"EVE_PUBLIC_IMPORT_DOMAIN_V1");
        hash.update(identity.domain);
        identity.domain = hash.finalize().into();
    }
    identity
}
