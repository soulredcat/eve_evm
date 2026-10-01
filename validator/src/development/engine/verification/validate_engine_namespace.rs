// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::config::{
    DevelopmentValidatorConfig, validate_development_validator_config,
};
use anyhow::{Result, ensure};
use std::path::{Component, Path};

pub(in crate::development::engine) fn validate_engine_namespace(
    config: &DevelopmentValidatorConfig,
    home: &Path,
) -> Result<()> {
    validate_development_validator_config(config)?;
    ensure!(
        home.is_absolute()
            && !home
                .components()
                .any(|part| matches!(part, Component::ParentDir | Component::CurDir)),
        "engine home must be a normal absolute task path"
    );
    let data = config.data.canonicalize()?;
    ensure!(
        home.parent()
            .ok_or_else(|| anyhow::anyhow!("engine home has no parent"))?
            .canonicalize()?
            == data,
        "engine home must be an immediate child of task data"
    );
    let metadata = std::fs::symlink_metadata(&config.data)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "engine data parent must be a real directory"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ensure!(
            metadata.uid() == rustix::process::geteuid().as_raw()
                && metadata.mode() & 0o777 == 0o700,
            "engine data requires actual current-UID Linux 0700 permissions"
        );
    }
    ensure!(
        config.rpc_address.port() != 0
            && config.p2p_address.port() != 0
            && config.rpc_address != config.p2p_address,
        "engine listeners require distinct nonzero ports"
    );
    Ok(())
}
