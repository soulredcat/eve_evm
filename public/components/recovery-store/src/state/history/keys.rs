// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) const INDEX_SCHEMA_KEY: &[u8] = b"eve/state/history-index/v1/schema";
pub(crate) const INDEX_SCHEMA: &[u8] = b"EVEHISTORY01";
pub(crate) const INDEX_CURSOR: &[u8] = b"eve/state/history-index/v1/cursor";
pub(crate) const BLOCK_LOOKUP: &[u8] = b"eve/state/history-index/v1/block-hash/";
pub(crate) const TX_LOOKUP: &[u8] = b"eve/state/history-index/v1/transaction/";
pub(crate) const VERSION_PREFIX: &[u8] = b"eve/state/history-index/v1/version/";
