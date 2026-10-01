// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use std::{collections::BTreeSet, io::ErrorKind};
pub(crate) fn count_owned_tcp_connections(pid: u32, local_port: u16) -> Result<usize> {
    let mut inodes = BTreeSet::new();
    for path in ["/proc/net/tcp", "/proc/net/tcp6"] {
        // Hosts without IPv6 have no tcp6 table and therefore no IPv6 sockets to count.
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if path.ends_with('6') && error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        };
        for line in text.lines().skip(1) {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.get(3) != Some(&"01") {
                continue;
            }
            let port = fields
                .get(1)
                .and_then(|local| local.rsplit_once(':'))
                .and_then(|(_, port)| u16::from_str_radix(port, 16).ok());
            if port == Some(local_port)
                && let Some(inode) = fields.get(9)
            {
                inodes.insert((*inode).to_owned());
            }
        }
    }
    let mut count = 0;
    for descriptor in std::fs::read_dir(format!("/proc/{pid}/fd"))?.flatten() {
        let Ok(link) = std::fs::read_link(descriptor.path()) else {
            continue;
        };
        if link
            .to_string_lossy()
            .strip_prefix("socket:[")
            .and_then(|value| value.strip_suffix(']'))
            .is_some_and(|inode| inodes.contains(inode))
        {
            count += 1;
        }
    }
    Ok(count)
}
