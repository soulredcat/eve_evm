// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{engine_file_identity, hash_engine_image_file, types::VerifiedEngineImage};
use crate::development::{
    config::DevelopmentValidatorConfig,
    engine::{
        commands::run_engine_command,
        types::{EXPECTED_NATIVE_VERSION, VerifiedEngineBinary},
    },
};
use anyhow::{Result, ensure};

pub(in crate::development::engine) fn verify_engine_binary(
    config: &DevelopmentValidatorConfig,
    expected: [u8; 32],
) -> Result<VerifiedEngineBinary> {
    let configured: [u8; 32] = hex::decode(&config.comet_sha256)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid engine digest width"))?;
    ensure!(
        expected != [0; 32] && configured == expected,
        "engine trusted launch/config digest mismatch"
    );
    let path = config.comet_binary.canonicalize()?;
    let mut file = std::fs::File::open(&path)?;
    let (digest, identity) = hash_engine_image_file(&mut file)?;
    ensure!(
        digest == expected,
        "engine executable differs from trusted source digest"
    );
    let version = run_engine_command(&path, &["version".into()], &config.data, "version")?;
    ensure!(
        std::str::from_utf8(&version)?.trim() == EXPECTED_NATIVE_VERSION,
        "engine native version/source revision mismatch"
    );
    ensure!(
        hash_engine_image_file(&mut file)? == (expected, identity)
            && engine_file_identity(&std::fs::File::open(&path)?)? == identity,
        "engine executable changed during provenance verification"
    );
    Ok(VerifiedEngineBinary {
        path,
        sha256: expected,
        image: VerifiedEngineImage {
            file,
            digest,
            identity,
        },
    })
}
