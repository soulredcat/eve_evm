// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::sync_node_directory;
use crate::consensus::runtime::types::NodeIdentity;
use anyhow::{Result, ensure};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    path::{Component, Path},
};

pub(in crate::consensus::runtime) fn open_node_namespace(
    data: &Path,
    identity: &NodeIdentity,
) -> Result<File> {
    ensure!(
        data.is_absolute()
            && !data
                .components()
                .any(|part| matches!(part, Component::CurDir | Component::ParentDir)),
        "node data requires a normal absolute Linux path"
    );
    let parent = data
        .parent()
        .ok_or_else(|| anyhow::anyhow!("node data parent missing"))?;
    ensure!(
        parent.canonicalize()? == parent,
        "node parent must not contain symlinks"
    );
    if !data.try_exists()? {
        std::fs::DirBuilder::new().mode(0o700).create(data)?;
        sync_node_directory(parent)?;
    }
    let metadata = std::fs::symlink_metadata(data)?;
    ensure!(
        metadata.is_dir()
            && metadata.uid() == rustix::process::geteuid().as_raw()
            && metadata.mode() & 0o777 == 0o700,
        "node data requires current-UID Linux 0700 directory"
    );
    let marker = data.join(".eve-validator.json");
    if !marker.try_exists()? {
        ensure!(
            std::fs::read_dir(data)?.next().is_none(),
            "unmarked nonempty node data cannot initialize"
        );
    }
    let lock = data.join(".eve-validator-owner.lock");
    if lock.try_exists()? {
        let meta = std::fs::symlink_metadata(&lock)?;
        ensure!(
            meta.is_file()
                && meta.len() == 0
                && meta.uid() == metadata.uid()
                && meta.mode() & 0o077 == 0,
            "foreign node ownership lease"
        );
    }
    let lease = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(lock)?;
    lease
        .try_lock()
        .map_err(|_| anyhow::anyhow!("node namespace already owned"))?;
    lease.sync_all()?;
    if marker.try_exists()? {
        let meta = std::fs::symlink_metadata(&marker)?;
        ensure!(
            meta.is_file()
                && meta.len() <= 4096
                && meta.uid() == metadata.uid()
                && meta.mode() & 0o077 == 0,
            "foreign node identity marker"
        );
        let mut bytes = Vec::new();
        File::open(&marker)?.take(4097).read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() <= 4096 && serde_json::from_slice::<NodeIdentity>(&bytes)? == *identity,
            "node namespace immutable identity mismatch"
        );
    } else {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(marker)?;
        file.write_all(&serde_json::to_vec(identity)?)?;
        file.sync_all()?;
    }
    for name in ["state", "signer", "replay", "engine"] {
        let child = data.join(name);
        if child.try_exists()? {
            let meta = std::fs::symlink_metadata(child)?;
            ensure!(
                meta.is_dir() && meta.uid() == metadata.uid() && meta.mode() & 0o777 == 0o700,
                "foreign or insecure existing node child namespace"
            );
        }
    }
    sync_node_directory(data)?;
    Ok(lease)
}
