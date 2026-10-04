// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod funded_genesis;
#[path = "genesis.rs"]
mod local_genesis;
pub use eve_state::DevelopmentGenesis as Genesis;
pub use funded_genesis::funded_genesis;
pub use local_genesis::genesis;
