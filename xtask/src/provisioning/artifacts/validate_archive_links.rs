// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::path::{Component, Path, PathBuf};

/// Inspect GNU tar's fixed numeric-owner/full-time listing, including every link target.
pub fn validate_archive_links(listing: &str, root: &str) -> Result<()> {
    ensure!(
        listing.len() <= 32 * 1024 * 1024,
        "archive metadata exceeds limit"
    );
    for line in listing.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        ensure!(fields.len() >= 6, "unrecognized archive metadata record");
        let kind = fields[0].as_bytes()[0];
        ensure!(
            b"-dlh".contains(&kind),
            "archive contains a special device/FIFO"
        );
        if kind != b'l' && kind != b'h' {
            ensure!(fields.len() == 6, "ambiguous archive metadata name");
            continue;
        }
        let target = if kind == b'l' {
            ensure!(
                fields.len() == 8 && fields[6] == "->",
                "unrecognized archive symlink metadata"
            );
            Path::new(fields[5])
                .parent()
                .unwrap_or(Path::new(""))
                .join(fields[7])
        } else {
            ensure!(
                fields.len() == 9 && fields[6] == "link" && fields[7] == "to",
                "unrecognized hard-link metadata"
            );
            PathBuf::from(fields[8])
        };
        let mut resolved = PathBuf::new();
        for part in target.components() {
            match part {
                Component::Normal(segment) => resolved.push(segment),
                Component::CurDir => {}
                Component::ParentDir => {
                    ensure!(
                        resolved.pop(),
                        "archive link traverses outside its source root"
                    );
                }
                _ => anyhow::bail!("archive link is absolute"),
            }
        }
        ensure!(
            resolved.starts_with(root),
            "archive link targets another source root"
        );
    }
    Ok(())
}
