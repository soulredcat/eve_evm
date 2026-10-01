// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    collect_module_path_attributes::collect_module_path_attributes,
    source_edge_inventory_adapter::SourceEdgeInventoryAdapter, types::SourceTarget,
};

pub fn record_source_module(visitor: &mut SourceEdgeInventoryAdapter<'_>, module: &syn::ItemMod) {
    let test_context = visitor.test_context
        || crate::structure::syntax::has_test_configuration::has_test_configuration(&module.attrs);
    let paths = match collect_module_path_attributes(&module.attrs) {
        Ok(paths) => paths,
        Err(error) => {
            visitor.inventory.violations.push(error.to_string());
            return;
        }
    };
    if module.content.is_none() {
        if !paths.has_unconditional_path {
            let ordinary = visitor
                .module_directory
                .join(format!("{}.rs", module.ident));
            let directory = visitor
                .module_directory
                .join(module.ident.to_string())
                .join("mod.rs");
            match (ordinary.exists(), directory.exists()) {
                (true, false) => visitor.inventory.targets.push(SourceTarget {
                    path: ordinary,
                    test_only: test_context,
                }),
                (false, true) => visitor.inventory.targets.push(SourceTarget {
                    path: directory,
                    test_only: test_context,
                }),
                _ => visitor.inventory.violations.push(format!(
                    "Module {} requires exactly one reviewed source target",
                    module.ident
                )),
            }
        }
        visitor
            .inventory
            .targets
            .extend(paths.paths.iter().map(|path| SourceTarget {
                path: visitor.attribute_directory.join(path),
                test_only: test_context,
            }));
        return;
    }
    let prior_attribute_directory = visitor.attribute_directory.clone();
    let prior_module_directory = visitor.module_directory.clone();
    let prior_test_context = visitor.test_context;
    let mut alternatives = if !paths.has_unconditional_path {
        vec![visitor.module_directory.join(module.ident.to_string())]
    } else {
        Vec::new()
    };
    alternatives.extend(
        paths
            .paths
            .iter()
            .map(|path| visitor.attribute_directory.join(path)),
    );
    for directory in alternatives {
        visitor.attribute_directory = directory.clone();
        visitor.module_directory = directory;
        visitor.test_context = test_context;
        syn::visit::visit_item_mod(visitor, module);
    }
    visitor.attribute_directory = prior_attribute_directory;
    visitor.module_directory = prior_module_directory;
    visitor.test_context = prior_test_context;
}
