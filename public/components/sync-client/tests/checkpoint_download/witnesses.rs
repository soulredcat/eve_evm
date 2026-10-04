// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::fixture,
    leases::reserve,
    native_fixture,
    server::{json, query, server},
};
use eve_finality_verifier::{
    CheckpointLimits, CheckpointWitnessWireKind, checkpoint_witness_wire_kind,
    preflight_checkpoint_witness_wire,
};
use eve_sync_client::{
    CheckpointWitnessHeights, downloaded_checkpoint_witness_wire, fetch_checkpoint_witness,
    fetch_native_frame, required_native_rpc_reservation,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

fn limits() -> CheckpointLimits {
    CheckpointLimits {
        maximum_height_gap: 64,
        maximum_witness_bytes: eve_finality_verifier::MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
    }
}
#[test]
fn actual_execution_and_exact_closing_lookahead_return_only_untrusted_canonical_wire() {
    let fixture = fixture();
    for (height, kind) in [
        (1, CheckpointWitnessWireKind::Execution),
        (2, CheckpointWitnessWireKind::Lookahead),
    ] {
        let mut replies = Vec::new();
        if height == 1 {
            replies.push(query(&fixture.encoded(&fixture.execution_response())));
        }
        replies.push(json(
            native_fixture::block(&fixture.chain, height),
            Duration::ZERO,
            "/block?",
        ));
        replies.push(json(
            native_fixture::commit(&fixture.chain, height),
            Duration::ZERO,
            "/commit?",
        ));
        let (rpc, peer) = server(replies);
        let held = Arc::new(AtomicUsize::new(0));
        let downloaded = fetch_checkpoint_witness(
            rpc,
            &fixture.chain.commits[0].target,
            CheckpointWitnessHeights {
                checkpoint: 1,
                witness: height as u64,
            },
            &fixture.limits.logical,
            limits(),
            &mut |bytes| reserve(&held, bytes),
        )
        .unwrap();
        peer.join().unwrap();
        let checked = preflight_checkpoint_witness_wire(
            downloaded_checkpoint_witness_wire(&downloaded),
            &fixture.limits.logical,
            limits(),
        )
        .unwrap();
        assert_eq!(checkpoint_witness_wire_kind(&checked), kind);
        assert!(held.load(Ordering::SeqCst) > 0);
        drop(downloaded);
        assert_eq!(held.load(Ordering::SeqCst), 0);
    }
}
#[test]
fn native_block_and_commit_fetch_do_not_reset_the_total_deadline() {
    let fixture = fixture();
    let (mut rpc, peer) = server(vec![
        json(
            native_fixture::block(&fixture.chain, 1),
            Duration::from_millis(300),
            "/block?",
        ),
        json(
            native_fixture::commit(&fixture.chain, 1),
            Duration::from_millis(300),
            "/commit?",
        ),
    ]);
    rpc.timeout = Duration::from_millis(500);
    let result = fetch_native_frame(rpc, 1, required_native_rpc_reservation(rpc).unwrap());
    peer.join().unwrap();
    assert!(result.is_err());
}
#[test]
fn wrong_witness_height_and_denied_final_codec_charge_refuse() {
    let fixture = fixture();
    let rpc = eve_sync_client::NativeRpcConfig {
        address: "127.0.0.1:1".parse().unwrap(),
        timeout: Duration::from_secs(1),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 131_072,
    };
    let mut calls = 0;
    assert!(
        fetch_checkpoint_witness(
            rpc,
            &fixture.chain.commits[0].target,
            CheckpointWitnessHeights {
                checkpoint: 1,
                witness: 3
            },
            &fixture.limits.logical,
            limits(),
            &mut |_| {
                calls += 1;
                Ok(())
            }
        )
        .is_err()
    );
    assert_eq!(calls, 0);
    let (rpc, peer) = server(vec![
        json(
            native_fixture::block(&fixture.chain, 2),
            Duration::ZERO,
            "/block?",
        ),
        json(
            native_fixture::commit(&fixture.chain, 2),
            Duration::ZERO,
            "/commit?",
        ),
    ]);
    let held = Arc::new(AtomicUsize::new(0));
    let mut calls = 0;
    let result = fetch_checkpoint_witness(
        rpc,
        &fixture.chain.commits[0].target,
        CheckpointWitnessHeights {
            checkpoint: 1,
            witness: 2,
        },
        &fixture.limits.logical,
        limits(),
        &mut |bytes| {
            calls += 1;
            if calls == 2 {
                anyhow::bail!("CALLER_CAPACITY");
            }
            reserve(&held, bytes)
        },
    );
    peer.join().unwrap();
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("CALLER_CAPACITY")
    );
    assert_eq!(calls, 2);
    assert_eq!(held.load(Ordering::SeqCst), 0);
}
