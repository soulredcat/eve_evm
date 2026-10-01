// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Pinned CometBFT wire boundary and commitment-height contracts.
//!
//! This component does not implement a consensus protocol or authenticate headers.

pub mod consensus;
pub mod wire;
