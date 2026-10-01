// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Pinned CometBFT wire boundary and commitment-height contracts.
//!
//! Classical certificate checks require a caller-authenticated historical set.
//! This component does not implement a consensus protocol or execute proposals.

pub mod consensus;
pub mod wire;
