// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    initialize_opaque_namespace, open_opaque_database, sync_opaque_directory,
    validate_opaque_namespace,
};
use crate::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity, OpaqueRecordRepository,
    hashing::hash_opaque_record, validate_opaque_record_budget,
};
use anyhow::{Context, Result, ensure};
use std::{io::ErrorKind, path::Path, sync::atomic::AtomicBool};

/// Only a nonexistent final directory may initialize. Existing incomplete namespaces never reset.
/// The existing parent and DB directory are synced before acknowledgment on the Unix reference.
pub fn open_opaque_record_repository(
    path: &Path,
    identity: OpaqueRecordIdentity,
    budget: OpaqueRecordBudget,
) -> Result<OpaqueRecordRepository> {
    validate_opaque_record_budget(&budget)?;
    ensure!(
        identity.genesis_hash != [0; 32] && identity.owner != [0; 32] && identity.domain != [0; 32],
        "empty opaque namespace identity/domain"
    );
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    ensure!(
        parent.is_dir(),
        "opaque namespace parent must already exist"
    );
    sync_opaque_directory(parent)?;
    let fresh = match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            ensure!(
                metadata.is_dir() && !metadata.file_type().is_symlink(),
                "opaque namespace must be a real directory"
            );
            false
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            std::fs::create_dir(path).context("exclusively create opaque namespace directory")?;
            sync_opaque_directory(parent)?;
            true
        }
        Err(error) => return Err(error.into()),
    };
    let database = open_opaque_database(path, &budget, fresh)?;
    let bootstrap = OpaqueRecordCursor {
        sequence: 0,
        content_hash: hash_opaque_record(identity, OpaqueRecordCursor::default(), 0, &[]),
    };
    if fresh {
        initialize_opaque_namespace(&database, identity, bootstrap, &budget)?;
    }
    let head = validate_opaque_namespace(&database, identity, bootstrap, &budget)?;
    sync_opaque_directory(path)?;
    sync_opaque_directory(parent)?;
    Ok(OpaqueRecordRepository {
        database,
        identity,
        budget,
        head,
        fenced: AtomicBool::new(false),
        #[cfg(test)]
        simulated_failure: None,
    })
}
