// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "delta/fixtures.rs"]
mod fixtures;
#[path = "delta/server.rs"]
mod server;
use eve_state::{Bytes, StateDeltaChunk, StateDeltaRequest, hash_state_delta_bytes};
use eve_sync_client::{fetch_state_delta_bytes, required_state_delta_download_reservation};

fn chunks() -> (StateDeltaRequest, Vec<StateDeltaChunk>) {
    let (parent, target) = fixtures::versions();
    let body = b"abcdefgh";
    let chunks = (0..2)
        .map(|index| StateDeltaChunk {
            parent: parent.clone(),
            target: target.clone(),
            durable_tip: target.clone(),
            body_sha256: hash_state_delta_bytes(body),
            total_length: body.len() as u64,
            offset: index * 4,
            data: Bytes::copy_from_slice(&body[index as usize * 4..index as usize * 4 + 4]),
        })
        .collect();
    (
        StateDeltaRequest {
            parent,
            target_height: 1,
            offset: 0,
            maximum_chunk_bytes: 4,
        },
        chunks,
    )
}
#[test]
fn actual_chunk_download_binds_exact_parent_order_and_complete_body_identity() {
    let (request, chunks) = chunks();
    let (config, worker) = server::server(chunks);
    let downloaded = fetch_state_delta_bytes(
        config,
        request.clone(),
        required_state_delta_download_reservation(config).unwrap(),
    )
    .unwrap();
    worker.join().unwrap();
    assert_eq!(downloaded.parent, request.parent);
    assert_eq!(downloaded.bytes, b"abcdefgh");
    assert_eq!(
        downloaded.body_sha256,
        hash_state_delta_bytes(&downloaded.bytes)
    );
    assert_eq!(downloaded.target.height, 1);
}
#[test]
fn changed_rehashed_chunk_identity_order_and_full_body_hash_refuse() {
    for variation in 0..3 {
        let (request, mut chunks) = chunks();
        match variation {
            0 => chunks[1].body_sha256[0] ^= 1,
            1 => chunks[1].offset = 0,
            _ => {
                chunks[0].body_sha256 = [9; 32];
                chunks[1].body_sha256 = [9; 32];
            }
        }
        let (config, worker) = server::server(chunks);
        assert!(
            fetch_state_delta_bytes(
                config,
                request,
                required_state_delta_download_reservation(config).unwrap()
            )
            .is_err()
        );
        worker.join().unwrap();
    }
}
#[test]
fn exact_parent_and_full_requested_chunk_width_refuse_before_next_request() {
    for variation in 0..2 {
        let (request, mut chunks) = chunks();
        chunks.truncate(1);
        match variation {
            0 => chunks[0].parent.timestamp += 1,
            _ => chunks[0].data = Bytes::from_static(b"ab"),
        }
        let (config, worker) = server::server(chunks);
        assert!(
            fetch_state_delta_bytes(
                config,
                request,
                required_state_delta_download_reservation(config).unwrap()
            )
            .is_err()
        );
        worker.join().unwrap();
    }
}
