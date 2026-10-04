// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::records::OpaqueRecordIdentity;
use sha2::{Digest, Sha256};

pub(super) fn derive_segmented_storage_identity(
    mut identity: OpaqueRecordIdentity,
) -> OpaqueRecordIdentity {
    let mut hash = Sha256::new();
    hash.update(b"EVE_PUBLIC_SEGMENTED_IMPORT_DOMAIN_V2");
    hash.update(identity.domain);
    identity.domain = hash.finalize().into();
    identity
}
