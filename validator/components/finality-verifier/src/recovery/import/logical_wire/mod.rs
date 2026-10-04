// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Explicit full logical import transport. It creates no finality or storage authority.

mod check_logical_component_lengths;
mod decode_logical_import_wire;
mod encode_logical_import_wire;
mod measure_logical_import_wire;
mod preflight_logical_import_wire;
mod slice_logical_import_wire;
mod types;
mod views;

pub use decode_logical_import_wire::decode_logical_import_wire;
pub use encode_logical_import_wire::encode_logical_import_wire;
pub use measure_logical_import_wire::measure_logical_import_wire;
pub use preflight_logical_import_wire::preflight_logical_import_wire;
pub use types::{LogicalImportWirePreflight, MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES};
pub use views::{
    logical_import_wire_budget, logical_import_wire_bytes, logical_import_wire_slices,
    logical_import_wire_stats,
};
