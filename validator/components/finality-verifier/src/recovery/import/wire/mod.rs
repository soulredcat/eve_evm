// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Compact canonical import bytes and bound borrowed resource preflight; no finality.
mod decoding;
mod encoding;
mod framing;
mod preflight;
mod types;
mod views;
pub(in crate::recovery::import) use preflight::{
    scan_import_execution_stats_with_limit, scan_import_lookahead_stats, scan_import_native_stats,
};

pub use decoding::decode_authenticated_import_wire;
pub use encoding::{encode_authenticated_import_wire, measure_authenticated_import_wire};
pub use preflight::preflight_authenticated_import_wire;
pub use types::{
    ImportExecutionWireStats, ImportLookaheadWireStats, ImportNativeWireStats, ImportWireError,
    ImportWirePreflight, ImportWireSlices, ImportWireStats, MAXIMUM_IMPORT_WIRE_BYTES,
};
pub use views::{import_wire_budget, import_wire_bytes, import_wire_slices, import_wire_stats};
