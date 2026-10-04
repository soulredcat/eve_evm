// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{MATERIALIZATION_DELAY, fixtures::versions, server::server};
use crate::{fetch_state_delta_bytes, required_state_delta_download_reservation};
use eve_state::{Bytes, StateDeltaChunk, StateDeltaRequest, hash_state_delta_bytes};
use std::time::Duration;

#[test]
fn one_final_chunk_that_materializes_after_total_deadline_is_refused() {
    let (parent, target) = versions();
    let bytes = b"bounded-body";
    let chunk = StateDeltaChunk {
        parent: parent.clone(),
        target: target.clone(),
        durable_tip: target,
        body_sha256: hash_state_delta_bytes(bytes),
        total_length: bytes.len() as u64,
        offset: 0,
        data: Bytes::from_static(bytes),
    };
    let (mut config, worker) = server(vec![chunk]);
    config.timeout = Duration::from_millis(500);
    // Model expensive final materialization after the actual bounded socket read.
    MATERIALIZATION_DELAY.with(|delay| delay.set(Duration::from_millis(600)));
    let result = fetch_state_delta_bytes(
        config,
        StateDeltaRequest {
            parent,
            target_height: 1,
            offset: 0,
            maximum_chunk_bytes: bytes.len() as u32,
        },
        required_state_delta_download_reservation(config).unwrap(),
    );
    MATERIALIZATION_DELAY.with(|delay| delay.set(Duration::ZERO));
    worker.join().unwrap();
    assert!(result.unwrap_err().to_string().contains("DEADLINE"));
}
