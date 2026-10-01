// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Ethereum-facing Shanghai execution header construction, separate from BFT IDs.

mod build_execution_header;
mod derive_next_base_fee;
mod types;

pub use build_execution_header::build_execution_header;
pub use derive_next_base_fee::derive_next_base_fee;
pub use types::{ExecutionHeaderInput, HeaderError};
