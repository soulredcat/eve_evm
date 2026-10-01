// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Child};

pub(crate) struct OwnedEngine {
    pub(super) child: Option<Child>,
    pub(super) image: super::verification::VerifiedEngineImage,
    pub(super) process: Option<rustix::fd::OwnedFd>,
    pub(super) lease: Option<super::home::EngineLease>,
    pub(super) signer_socket: PathBuf,
    pub(super) data: PathBuf,
    pub(super) output_log: PathBuf,
    pub(super) error_log: PathBuf,
}

pub(super) struct VerifiedEngineBinary {
    pub path: PathBuf,
    pub sha256: [u8; 32],
    pub image: super::verification::VerifiedEngineImage,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct EngineHomeMarker {
    pub schema: u32,
    pub stage: String,
    pub binary_sha256: String,
    pub native_version: String,
    pub genesis_sha256: String,
    pub configuration_sha256: String,
    pub node_key_sha256: String,
    pub dummy_key_sha256: String,
}

pub(super) const EXPECTED_NATIVE_VERSION: &str = "0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62";
pub(super) const MAXIMUM_ENGINE_BINARY_BYTES: u64 = 512 * 1_048_576;
pub(super) const MARKER_NAME: &str = ".eve-engine.json";
