// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod commits;
mod opening;
mod state_reader;
pub(crate) mod validation;
pub use commits::commit_state;
pub(crate) use opening::open_state_namespace;
pub use opening::open_state_repository;
pub use state_reader::state_reader;
