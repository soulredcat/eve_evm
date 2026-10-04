// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, fixture, lookahead};
use eve_finality_verifier::{
    CheckpointWitness, CheckpointWitnessWireKind, checkpoint_witness_wire_bytes,
    checkpoint_witness_wire_kind, checkpoint_witness_wire_stats,
    checkpoint_witness_wire_version_bytes, decode_checkpoint_witness_wire,
    encode_checkpoint_witness_wire, measure_checkpoint_witness_wire,
    preflight_checkpoint_witness_wire, required_checkpoint_witness_decode_reservation,
};
use eve_state::encode_state_version;

#[test]
fn execution_and_lookahead_roundtrip_exact_canonical_native_and_state_bytes() {
    let (chain, budget, limits) = fixture();
    for input in [execution(&chain), lookahead(&chain)] {
        let bytes = encode_checkpoint_witness_wire(&input, &budget, limits).unwrap();
        assert_eq!(
            bytes.len(),
            measure_checkpoint_witness_wire(&input, &budget, limits).unwrap()
        );
        let preflight = preflight_checkpoint_witness_wire(&bytes, &budget, limits).unwrap();
        assert_eq!(checkpoint_witness_wire_bytes(&preflight), bytes);
        let stats = checkpoint_witness_wire_stats(&preflight);
        assert_eq!(stats.encoded_bytes, bytes.len());
        assert!(stats.native.frame.signature_count > 0);
        let required = required_checkpoint_witness_decode_reservation(&preflight).unwrap();
        let decoded = decode_checkpoint_witness_wire(&preflight, required).unwrap();
        assert_eq!(
            encode_checkpoint_witness_wire(&decoded, &budget, limits).unwrap(),
            bytes
        );
        match (&input, decoded) {
            (CheckpointWitness::Execution(expected), CheckpointWitness::Execution(actual)) => {
                assert_eq!(
                    checkpoint_witness_wire_kind(&preflight),
                    CheckpointWitnessWireKind::Execution
                );
                assert_eq!(actual.native, expected.native);
                assert_eq!(actual.block, expected.block);
                assert_eq!(actual.version, expected.version);
                assert_eq!(
                    stats.version_network_bytes,
                    expected.version.identity.network_name.len()
                );
                assert_eq!(
                    checkpoint_witness_wire_version_bytes(&preflight).unwrap(),
                    encode_state_version(&expected.version).unwrap().as_ref()
                );
                assert_eq!(
                    stats.execution.unwrap().transaction_count,
                    expected.block.transactions.len()
                );
            }
            (CheckpointWitness::Lookahead(expected), CheckpointWitness::Lookahead(actual)) => {
                assert_eq!(
                    checkpoint_witness_wire_kind(&preflight),
                    CheckpointWitnessWireKind::Lookahead
                );
                assert_eq!(&actual, expected);
                assert!(stats.execution.is_none());
                assert!(checkpoint_witness_wire_version_bytes(&preflight).is_none());
            }
            _ => panic!("wire kind changed"),
        }
    }
}
