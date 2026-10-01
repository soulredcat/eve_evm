// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod append_history_index;
mod encode_index_entries;
mod ensure_history_index;
pub(crate) use append_history_index::append_history_index;
pub(crate) use encode_index_entries::encode_index_entries;
pub use ensure_history_index::ensure_history_index;
