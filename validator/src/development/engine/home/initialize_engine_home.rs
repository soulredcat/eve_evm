// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    acquire_engine_lease, sync_engine_directory, validate_engine_home, write_owned_engine_file,
};
use crate::development::{
    config::DevelopmentValidatorConfig,
    engine::{
        commands::run_engine_command,
        configuration::validate_engine_public_genesis,
        types::{EXPECTED_NATIVE_VERSION, EngineHomeMarker, MARKER_NAME},
        verification::{
            hash_engine_file, read_engine_file, validate_engine_namespace, verify_engine_binary,
        },
    },
};
use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(crate) fn initialize_engine_home(
    config: &DevelopmentValidatorConfig,
    home: &Path,
    native_genesis: &[u8],
    expected_sha256: [u8; 32],
) -> Result<()> {
    validate_engine_namespace(config, home)?;
    let genesis = validate_engine_public_genesis(native_genesis)?;
    let _lease = acquire_engine_lease(&config.data)?;
    let binary = verify_engine_binary(config, expected_sha256)?;
    let genesis_digest = hex::encode(Sha256::digest(native_genesis));
    if home.try_exists()? {
        let marker = validate_engine_home(home, &binary)?;
        ensure!(
            marker.genesis_sha256 == genesis_digest,
            "existing engine home cannot change immutable genesis"
        );
        return Ok(());
    }
    std::fs::create_dir(home)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(home, std::fs::Permissions::from_mode(0o700))?;
    }
    sync_engine_directory(&config.data)?;
    let mut marker = EngineHomeMarker {
        schema: 1,
        stage: "initializing".into(),
        binary_sha256: hex::encode(binary.sha256),
        native_version: EXPECTED_NATIVE_VERSION.into(),
        genesis_sha256: genesis_digest,
        configuration_sha256: String::new(),
        node_key_sha256: String::new(),
        dummy_key_sha256: String::new(),
    };
    write_owned_engine_file(&home.join(MARKER_NAME), &serde_json::to_vec(&marker)?, 4096)?;
    run_engine_command(
        &binary.path,
        &[
            "init".into(),
            "--home".into(),
            home.as_os_str().to_owned(),
            "--log_level".into(),
            "error".into(),
        ],
        &config.data,
        "init",
    )?;
    let dummy: serde_json::Value = serde_json::from_slice(&read_engine_file(
        &home.join("config/priv_validator_key.json"),
        16_384,
    )?)?;
    let dummy_address = dummy["address"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("generated dummy native PV address missing"))?;
    ensure!(
        genesis["validators"]
            .as_array()
            .is_some_and(|members| members
                .iter()
                .all(|member| member["address"].as_str() != Some(dummy_address))),
        "generated dummy native key must never enroll as EVE validator"
    );
    write_owned_engine_file(&home.join("config/genesis.json"), native_genesis, 1_048_576)?;
    marker.configuration_sha256 =
        hex::encode(hash_engine_file(&home.join("config/config.toml"), 65_536)?);
    marker.node_key_sha256 = hex::encode(hash_engine_file(
        &home.join("config/node_key.json"),
        16_384,
    )?);
    marker.dummy_key_sha256 = hex::encode(hash_engine_file(
        &home.join("config/priv_validator_key.json"),
        16_384,
    )?);
    super::sync_native_engine_files(home)?;
    marker.stage = "ready".into();
    write_owned_engine_file(&home.join(MARKER_NAME), &serde_json::to_vec(&marker)?, 4096)?;
    sync_engine_directory(home)?;
    sync_engine_directory(&config.data)?;
    validate_engine_home(home, &binary)?;
    Ok(())
}
