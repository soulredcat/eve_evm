// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

pub fn collect_use_tree(tree: &syn::UseTree, prefix: &str, imports: &mut BTreeMap<String, String>) {
    match tree {
        syn::UseTree::Path(path) => {
            let next = if prefix.is_empty() {
                path.ident.to_string()
            } else {
                format!("{prefix}::{}", path.ident)
            };
            collect_use_tree(&path.tree, &next, imports);
        }
        syn::UseTree::Name(name) => {
            let target = if prefix.is_empty() {
                name.ident.to_string()
            } else {
                format!("{prefix}::{}", name.ident)
            };
            imports.insert(name.ident.to_string(), target);
        }
        syn::UseTree::Rename(rename) => {
            let target = if prefix.is_empty() {
                rename.ident.to_string()
            } else {
                format!("{prefix}::{}", rename.ident)
            };
            imports.insert(rename.rename.to_string(), target);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_tree(item, prefix, imports);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
