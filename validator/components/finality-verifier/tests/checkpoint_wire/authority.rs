// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, fixture};
use eve_finality_verifier::{
    decode_checkpoint_witness_wire, encode_checkpoint_witness_wire,
    preflight_checkpoint_witness_wire, required_checkpoint_witness_decode_reservation,
};

#[test]
fn canonical_codec_does_not_treat_a_shaped_signature_as_authentication() {
    let (chain, budget, limits) = fixture();
    let mut input = execution(&chain);
    let eve_finality_verifier::CheckpointWitness::Execution(witness) = &mut input else {
        unreachable!()
    };
    witness.native.frame.commit.signatures[0].signature[0] ^= 1;
    let bytes = encode_checkpoint_witness_wire(&input, &budget, limits).unwrap();
    let preflight = preflight_checkpoint_witness_wire(&bytes, &budget, limits).unwrap();
    let reserved = required_checkpoint_witness_decode_reservation(&preflight).unwrap();
    let decoded = decode_checkpoint_witness_wire(&preflight, reserved).unwrap();
    assert_eq!(
        encode_checkpoint_witness_wire(&decoded, &budget, limits).unwrap(),
        bytes
    );
    // The return type is still untrusted CheckpointWitness; only the verifier can authorize it.
}
