// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::fixture,
    leases::reserve,
    server::{query, server},
};
use eve_storage::checkpoints::messages::{CheckpointRequestKind, CheckpointResponse};
use eve_sync_client::{downloaded_checkpoint_response, fetch_checkpoint_response};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn actual_manifest_chunk_and_execution_queries_match_requests_and_hold_leases() {
    let fixture = fixture();
    let cases = [
        (
            CheckpointRequestKind::Manifest { chunk_bytes: 64 },
            fixture.manifest_response(),
        ),
        (
            CheckpointRequestKind::Chunk {
                chunk_bytes: 64,
                manifest_id: fixture.id,
                index: 0,
            },
            fixture.chunk_response(0),
        ),
        (
            CheckpointRequestKind::Execution,
            fixture.execution_response(),
        ),
    ];
    for (kind, expected) in cases {
        let request = fixture.request(kind);
        let (rpc, peer) = server(vec![query(&fixture.encoded(&expected))]);
        let held = Arc::new(AtomicUsize::new(0));
        let downloaded = fetch_checkpoint_response(rpc, &request, &fixture.limits, &mut |bytes| {
            reserve(&held, bytes)
        })
        .unwrap();
        peer.join().unwrap();
        assert_eq!(downloaded_checkpoint_response(&downloaded), &expected);
        assert!(held.load(Ordering::SeqCst) > 0);
        drop(downloaded);
        assert_eq!(held.load(Ordering::SeqCst), 0);
    }
}
#[test]
fn wrong_height_kind_network_manifest_index_and_chunk_width_are_refused() {
    let fixture = fixture();
    let wrong_height = CheckpointResponse::Execution {
        target: fixture.chain.commits[2].target.clone(),
        block: Box::new(fixture.chain.commits[2].block.clone()),
    };
    let wrong_network = super::foreign_response::foreign_execution_response();
    let cases = [
        (CheckpointRequestKind::Execution, wrong_height, "HEIGHT"),
        (CheckpointRequestKind::Execution, wrong_network, "IDENTITY"),
        (
            CheckpointRequestKind::Execution,
            fixture.chunk_response(0),
            "KIND",
        ),
        (
            CheckpointRequestKind::Manifest { chunk_bytes: 128 },
            fixture.manifest_response(),
            "CHUNK_WIDTH",
        ),
        (
            CheckpointRequestKind::Chunk {
                chunk_bytes: 64,
                manifest_id: [0x92; 32],
                index: 0,
            },
            fixture.chunk_response(0),
            "CHUNK_IDENTITY",
        ),
        (
            CheckpointRequestKind::Chunk {
                chunk_bytes: 64,
                manifest_id: fixture.id,
                index: 1,
            },
            fixture.chunk_response(0),
            "CHUNK_IDENTITY",
        ),
        (
            CheckpointRequestKind::Chunk {
                chunk_bytes: 128,
                manifest_id: fixture.id,
                index: 0,
            },
            fixture.chunk_response(0),
            "CHUNK_IDENTITY",
        ),
    ];
    for (kind, response, reason) in cases {
        let (rpc, peer) = server(vec![query(&fixture.encoded(&response))]);
        let held = Arc::new(AtomicUsize::new(0));
        let result =
            fetch_checkpoint_response(rpc, &fixture.request(kind), &fixture.limits, &mut |bytes| {
                reserve(&held, bytes)
            });
        peer.join().unwrap();
        assert!(result.err().unwrap().to_string().contains(reason));
        assert_eq!(held.load(Ordering::SeqCst), 0);
    }
}
#[test]
fn truncated_trailing_and_transport_oversize_refuse_without_leaked_leases() {
    let fixture = fixture();
    let good = fixture.encoded(&fixture.execution_response());
    let mut trailing = good.clone();
    trailing.push(0);
    for (bytes, ceiling) in [
        (good[..good.len() - 1].to_vec(), 131_072),
        (trailing, 131_072),
        (good, 256),
    ] {
        let (mut rpc, peer) = server(vec![query(&bytes)]);
        rpc.maximum_response_bytes = ceiling;
        let held = Arc::new(AtomicUsize::new(0));
        assert!(
            fetch_checkpoint_response(
                rpc,
                &fixture.request(CheckpointRequestKind::Execution),
                &fixture.limits,
                &mut |bytes| reserve(&held, bytes)
            )
            .is_err()
        );
        peer.join().unwrap();
        assert_eq!(held.load(Ordering::SeqCst), 0);
    }
}
