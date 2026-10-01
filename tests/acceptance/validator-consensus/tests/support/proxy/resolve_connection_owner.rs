// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ProxyState;
use anyhow::{Result, ensure};
use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

pub(super) fn resolve_connection_owner(state: &ProxyState, port: u16) -> Result<usize> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let tcp = std::fs::read_to_string("/proc/net/tcp")?;
        let inodes: BTreeSet<_> = tcp
            .lines()
            .skip(1)
            .filter_map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                let local = fields.get(1)?.rsplit_once(':')?;
                (u16::from_str_radix(local.1, 16).ok()? == port)
                    .then(|| fields.get(9).copied())
                    .flatten()
            })
            .map(str::to_owned)
            .collect();
        for (index, pid) in state.owners.lock().unwrap().iter().enumerate() {
            let Some(pid) = pid else {
                continue;
            };
            let Ok(descriptors) = std::fs::read_dir(format!("/proc/{pid}/fd")) else {
                continue;
            };
            for descriptor in descriptors.flatten() {
                let Ok(link) = std::fs::read_link(descriptor.path()) else {
                    continue;
                };
                let value = link.to_string_lossy();
                if value
                    .strip_prefix("socket:[")
                    .and_then(|text| text.strip_suffix(']'))
                    .is_some_and(|inode| inodes.contains(inode))
                {
                    return Ok(index);
                }
            }
        }
        ensure!(
            Instant::now() < deadline,
            "P2P source not an owned native engine socket"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}
