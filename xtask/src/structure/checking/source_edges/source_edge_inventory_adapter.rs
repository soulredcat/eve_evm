// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{record_source_module::record_source_module, types::SourceEdgeInventory};
use std::path::PathBuf;
use syn::visit::Visit;

pub struct SourceEdgeInventoryAdapter<'a> {
    pub inventory: &'a mut SourceEdgeInventory,
    pub attribute_directory: PathBuf,
    pub module_directory: PathBuf,
    pub test_context: bool,
}

impl<'ast> Visit<'ast> for SourceEdgeInventoryAdapter<'_> {
    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        record_source_module(self, module);
    }
}
