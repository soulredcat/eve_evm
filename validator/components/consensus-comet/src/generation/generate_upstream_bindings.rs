// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{error::Error, fs, path::PathBuf};

use sha2::{Digest, Sha256};

/// Check exact vendored source bytes before generating untracked Rust bindings.
pub fn generate_upstream_bindings() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let checksums = root.join("vendor/SHA256SUMS");
    println!("cargo:rerun-if-changed={}", checksums.display());
    for line in fs::read_to_string(checksums)?.lines() {
        let (expected, relative) = line.split_once("  ").ok_or("invalid vendor checksum row")?;
        if relative.contains("..") || !relative.starts_with("vendor/") {
            return Err("vendor checksum path escapes the package".into());
        }
        let path = root.join(relative);
        let actual: String = Sha256::digest(fs::read(&path)?)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        if actual != expected {
            return Err(format!("vendored source digest mismatch: {relative}").into());
        }
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let proto = root.join("vendor/cometbft/proto");
    let gogo = root.join("vendor/gogoproto");
    let well_known = protoc_bin_vendored::include_path()?;
    prost_build::Config::new()
        .protoc_executable(protoc_bin_vendored::protoc_bin_path()?)
        .compile_protos(
            &[
                proto.join("tendermint/abci/types.proto"),
                proto.join("tendermint/types/canonical.proto"),
            ],
            &[proto, gogo, well_known],
        )?;
    Ok(())
}
