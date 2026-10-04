// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded canonical witness transport. Parsing creates no authenticated checkpoint authority.
mod decode_checkpoint_witness_wire;
mod encode_checkpoint_witness_wire;
mod estimate_checkpoint_witness_decode_charge;
mod measure_checkpoint_witness_wire;
mod preflight_checkpoint_witness_wire;
mod required_checkpoint_witness_decode_reservation;
mod types;
mod validate_checkpoint_witness_wire_limits;
mod views;
pub use decode_checkpoint_witness_wire::decode_checkpoint_witness_wire;
pub use encode_checkpoint_witness_wire::encode_checkpoint_witness_wire;
pub use measure_checkpoint_witness_wire::measure_checkpoint_witness_wire;
pub use preflight_checkpoint_witness_wire::preflight_checkpoint_witness_wire;
pub use required_checkpoint_witness_decode_reservation::required_checkpoint_witness_decode_reservation;
pub use types::{
    CheckpointWitnessWireError, CheckpointWitnessWireKind, CheckpointWitnessWirePreflight,
    CheckpointWitnessWireStats, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
};
pub use views::{
    checkpoint_witness_wire_budget, checkpoint_witness_wire_bytes, checkpoint_witness_wire_kind,
    checkpoint_witness_wire_limits, checkpoint_witness_wire_stats,
    checkpoint_witness_wire_version_bytes,
};
#[cfg(test)]
mod tests;
