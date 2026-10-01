// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, address};

/// Reserved native interface and staking custody address; development only.
pub const SYSTEM_INTERFACE_ADDRESS: Address = address!("000000000000000000000000000000000000f100");
/// Funded node-reward escrow; no externally signed spending authority.
pub const NODE_POOL_ADDRESS: Address = address!("000000000000000000000000000000000000f101");
/// Funded validator-reward escrow; no externally signed spending authority.
pub const VALIDATOR_POOL_ADDRESS: Address = address!("000000000000000000000000000000000000f102");
