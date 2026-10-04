// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, fixture};
use eve_finality_verifier::{
    checkpoint_witness_wire_bytes, checkpoint_witness_wire_stats, decode_checkpoint_witness_wire,
    encode_checkpoint_witness_wire, preflight_checkpoint_witness_wire,
    required_checkpoint_witness_decode_reservation,
};

#[test]
fn detached_statistics_and_owned_decode_cannot_rebind_the_sealed_borrowed_input() {
    let (chain, budget, limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    let preflight = preflight_checkpoint_witness_wire(&bytes, &budget, limits).unwrap();
    let mut detached = checkpoint_witness_wire_stats(&preflight);
    detached.encoded_bytes = 0;
    detached.native.transaction_count = 0;
    assert_eq!(detached.encoded_bytes, 0);
    assert_eq!(detached.native.transaction_count, 0);
    assert_eq!(
        checkpoint_witness_wire_bytes(&preflight).as_ptr(),
        bytes.as_ptr()
    );
    assert_eq!(
        checkpoint_witness_wire_stats(&preflight).encoded_bytes,
        bytes.len()
    );
    let required = required_checkpoint_witness_decode_reservation(&preflight).unwrap();
    let mut decoded = decode_checkpoint_witness_wire(&preflight, required).unwrap();
    let eve_finality_verifier::CheckpointWitness::Execution(execution) = &mut decoded else {
        unreachable!()
    };
    execution.version.height += 1;
    assert_eq!(checkpoint_witness_wire_bytes(&preflight), bytes);
    assert_eq!(
        checkpoint_witness_wire_stats(&preflight).encoded_bytes,
        bytes.len()
    );
}
