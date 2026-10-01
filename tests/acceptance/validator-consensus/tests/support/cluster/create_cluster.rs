// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Cluster, ClusterOptions, Node};
use crate::support::{
    fixture::write_cluster_genesis, process::write_private_file, proxy::ProxySet,
};
use anyhow::{Context, Result, ensure};
use ed25519_dalek::SigningKey;
use k256::elliptic_curve::rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::{net::TcpListener, os::unix::fs::PermissionsExt, path::PathBuf};

impl Cluster {
    pub fn create(name: &str, options: ClusterOptions) -> Result<Self> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .context("repository root")?
            .to_path_buf();
        let artifacts = root.join("local-tests/b3-preparation");
        std::fs::create_dir_all(&artifacts)?;
        let artifact = tempfile::Builder::new()
            .prefix(&format!("{name}-"))
            .tempdir_in(artifacts)?
            .keep();
        std::fs::set_permissions(&artifact, std::fs::Permissions::from_mode(0o700))?;
        let namespace = tempfile::Builder::new().prefix("eve-b3-").tempdir()?;
        std::fs::set_permissions(namespace.path(), std::fs::Permissions::from_mode(0o700))?;
        let binary = PathBuf::from(
            std::env::var_os("EVE_VALIDATOR_DEV_BINARY")
                .context("EVE_VALIDATOR_DEV_BINARY required")?,
        );
        let comet = PathBuf::from(std::env::var_os("EVE_COMET").context("EVE_COMET required")?);
        let comet_sha = std::env::var("EVE_COMET_SHA256").context("EVE_COMET_SHA256 required")?;
        let binary_sha256 = hex::encode(Sha256::digest(std::fs::read(&binary)?));
        ensure!(
            hex::encode(Sha256::digest(std::fs::read(&comet)?)) == comet_sha,
            "pinned native engine identity mismatch"
        );
        let count = if options.observer { 5 } else { 4 };
        let proxies = ProxySet::create(count)?;
        let mut reservations = Vec::new();
        let mut nodes = Vec::new();
        for index in 0..count {
            let mut seed = [0; 32];
            OsRng.fill_bytes(&mut seed);
            let key = SigningKey::from_bytes(&seed);
            let data = namespace.path().join(format!("n{index}"));
            let seed_path = namespace.path().join(format!("k{index}"));
            write_private_file(&seed_path, &seed)?;
            seed.fill(0);
            let rpc = TcpListener::bind("127.0.0.1:0")?;
            let p2p = TcpListener::bind("127.0.0.1:0")?;
            nodes.push(Node {
                data,
                seed: seed_path,
                rpc: rpc.local_addr()?,
                p2p: p2p.local_addr()?,
                advertised: proxies.addresses[index],
                node_id: String::new(),
                public_key: key.verifying_key().to_bytes(),
                owner: alloy_primitives::Address::repeat_byte(if index == 4 {
                    1
                } else {
                    (index + 1) as u8
                }),
                child: None,
                engine_pid: None,
                engine_start: None,
            });
            reservations.extend([rpc, p2p]);
        }
        let (genesis_path, genesis, authority, authority_address, fixture_path, fixture_digest) =
            write_cluster_genesis(&artifact, &nodes, &options)?;
        let mut cluster = Self {
            namespace: Some(namespace),
            retain_failed_namespace: false,
            binary_sha256,
            artifact,
            binary,
            comet,
            comet_sha,
            genesis_path,
            genesis,
            chain_id: String::new(),
            authority,
            authority_address,
            fixture_path,
            fixture_digest,
            nodes,
            proxies,
            reservations,
        };
        cluster.initialize()?;
        Ok(cluster)
    }
}
