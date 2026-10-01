// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod build_fixture_configuration;
mod write_cluster_genesis;
pub(crate) use write_cluster_genesis::write_cluster_genesis;
pub(crate) const TRANSITION_ADDRESS: &str = "000000000000000000000000000000000000f1b3";
pub(crate) const REVERT_ADDRESS: &str = "0000000000000000000000000000000000000042";
