// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{hash_engine_image_file, types::VerifiedEngineImage};
use anyhow::{Result, ensure};

/// Actual-byte test fixture only; this constructor is absent from production builds.
pub(crate) fn verify_test_engine_image(
    path: &std::path::Path,
    expected: [u8; 32],
) -> Result<VerifiedEngineImage> {
    let mut file = std::fs::File::open(path)?;
    let (digest, identity) = hash_engine_image_file(&mut file)?;
    ensure!(
        digest == expected && expected != [0; 32],
        "test image digest mismatch"
    );
    Ok(VerifiedEngineImage {
        file,
        digest,
        identity,
    })
}
