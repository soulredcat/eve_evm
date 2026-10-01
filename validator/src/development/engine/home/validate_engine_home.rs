// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_engine_home_marker;
use crate::development::engine::{
    types::{EXPECTED_NATIVE_VERSION, EngineHomeMarker, VerifiedEngineBinary},
    verification::{hash_engine_file, read_engine_file},
};
use anyhow::{Result, ensure};
use std::path::Path;

pub(in crate::development::engine) fn validate_engine_home(
    home: &Path,
    binary: &VerifiedEngineBinary,
) -> Result<EngineHomeMarker> {
    let metadata = std::fs::symlink_metadata(home)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "engine home must be a real owned directory"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ensure!(
            metadata.uid() == rustix::process::geteuid().as_raw()
                && metadata.mode() & 0o777 == 0o700,
            "engine home requires real private current-UID permissions"
        );
    }
    for directory in ["config", "data"] {
        let metadata = std::fs::symlink_metadata(home.join(directory))?;
        ensure!(
            metadata.is_dir() && !metadata.file_type().is_symlink(),
            "engine home subdirectory must be real and owned"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            ensure!(
                metadata.uid() == rustix::process::geteuid().as_raw()
                    && metadata.mode() & 0o777 == 0o700,
                "engine home subdirectory requires actual private permissions"
            );
        }
    }
    for file in [
        ".eve-engine.json",
        "config/genesis.json",
        "config/config.toml",
    ] {
        let metadata = std::fs::symlink_metadata(home.join(file))?;
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink(),
            "engine home provenance/configuration path must be a regular file"
        );
    }
    let marker = read_engine_home_marker(home)?;
    ensure!(
        marker.binary_sha256 == hex::encode(binary.sha256)
            && marker.native_version == EXPECTED_NATIVE_VERSION,
        "engine home executable provenance mismatch"
    );
    ensure!(
        marker.genesis_sha256
            == hex::encode(hash_engine_file(
                &home.join("config/genesis.json"),
                1_048_576
            )?)
            && marker.configuration_sha256
                == hex::encode(hash_engine_file(&home.join("config/config.toml"), 65_536)?),
        "engine home immutable genesis/previous configuration mismatch"
    );
    ensure!(
        marker.node_key_sha256
            == hex::encode(hash_engine_file(
                &home.join("config/node_key.json"),
                16_384
            )?)
            && marker.dummy_key_sha256
                == hex::encode(hash_engine_file(
                    &home.join("config/priv_validator_key.json"),
                    16_384
                )?),
        "engine native P2P/dummy-key identity changed"
    );
    for file in [
        "config/node_key.json",
        "config/priv_validator_key.json",
        "data/priv_validator_state.json",
    ] {
        let metadata = std::fs::symlink_metadata(home.join(file))?;
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink(),
            "native generated key/state path must not be a symlink"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            ensure!(
                metadata.uid() == rustix::process::geteuid().as_raw()
                    && metadata.mode() & 0o777 == 0o600,
                "native generated key/state requires actual owned 0600 permissions"
            );
        }
        ensure!(
            !read_engine_file(&home.join(file), 16_384)?.is_empty(),
            "engine home generated native prerequisite missing"
        );
    }
    Ok(marker)
}
