// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::open_proof_file::open_proof_file;
use crate::sync::{resources::reserve_master_bytes, types::ChargedProof};
use anyhow::{Result, ensure};
use std::{fs::File, io::Read, sync::Arc};
use tokio::sync::Semaphore;

/// Charge the actual regular-file length before any payload allocation. A growing
/// or shrinking file rejects rather than creating an unbounded read_to_end Vec.
pub(in crate::sync) fn read_charged_proof(
    directory: &File,
    name: &str,
    maximum: usize,
    pool: &Arc<Semaphore>,
) -> Result<ChargedProof> {
    let mut file = open_proof_file(directory, name, false)?;
    let length = usize::try_from(file.metadata()?.len())?;
    ensure!(length <= maximum, "MASTER_PROOF_BYTE_CAPACITY");
    let lease = reserve_master_bytes(pool, length.max(1))?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(length)?;
    ensure!(
        bytes.capacity() <= length.max(1),
        "MASTER_PROOF_ALLOCATION_CAPACITY"
    );
    bytes.resize(length, 0);
    file.read_exact(&mut bytes)?;
    ensure!(
        file.read(&mut [0_u8; 1])? == 0 && file.metadata()?.len() == length as u64,
        "MASTER_PROOF_CHANGED_DURING_READ"
    );
    Ok(ChargedProof {
        bytes,
        _lease: lease,
    })
}
