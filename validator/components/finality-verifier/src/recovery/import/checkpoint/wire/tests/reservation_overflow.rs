// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    CheckpointWitnessWireError, CheckpointWitnessWireStats,
    estimate_checkpoint_witness_decode_charge::estimate_checkpoint_witness_decode_charge,
};
use crate::{ImportLookaheadWireStats, ImportNativeWireStats};

fn statistics() -> CheckpointWitnessWireStats {
    CheckpointWitnessWireStats {
        encoded_bytes: 0,
        version_encoded_bytes: 0,
        version_network_bytes: 0,
        execution: None,
        native: ImportLookaheadWireStats {
            encoded_bytes: 0,
            transaction_count: 0,
            transaction_bytes: 0,
            frame: ImportNativeWireStats {
                encoded_bytes: 0,
                block_id_encoded_bytes: 0,
                header_encoded_bytes: 0,
                commit_encoded_bytes: 0,
                signature_count: 0,
            },
        },
    }
}
#[test]
fn reservation_rejects_encoded_signature_element_and_raw_overflow() {
    for field in 0..6 {
        let mut stats = statistics();
        match field {
            0 => stats.encoded_bytes = usize::MAX,
            1 => stats.native.frame.signature_count = usize::MAX,
            2 => stats.native.transaction_count = usize::MAX,
            3 => stats.native.transaction_bytes = usize::MAX,
            4 => stats.version_encoded_bytes = usize::MAX,
            _ => stats.version_network_bytes = usize::MAX,
        }
        assert_eq!(
            estimate_checkpoint_witness_decode_charge(stats),
            Err(CheckpointWitnessWireError::ArithmeticOverflow)
        );
    }
}
