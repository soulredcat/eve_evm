// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::syntax::is_macro_expression::is_macro_expression;
use syn::Pat;
pub(super) fn is_bounded_pattern(pattern: &Pat) -> bool {
    match pattern {
        Pat::Wild(_) | Pat::Path(_) | Pat::Rest(_) | Pat::Lit(_) => true,
        Pat::Ident(value) => value
            .subpat
            .as_ref()
            .is_none_or(|(_, value)| is_bounded_pattern(value)),
        Pat::Or(value) => value.cases.iter().all(is_bounded_pattern),
        Pat::Paren(value) => is_bounded_pattern(&value.pat),
        Pat::Reference(value) => is_bounded_pattern(&value.pat),
        Pat::Slice(value) => value.elems.iter().all(is_bounded_pattern),
        Pat::Tuple(value) => value.elems.iter().all(is_bounded_pattern),
        Pat::TupleStruct(value) => value.elems.iter().all(is_bounded_pattern),
        Pat::Struct(value) => value
            .fields
            .iter()
            .all(|field| is_bounded_pattern(&field.pat)),
        Pat::Type(value) => is_bounded_pattern(&value.pat),
        Pat::Range(value) => {
            value.start.as_deref().is_none_or(is_macro_expression)
                && value.end.as_deref().is_none_or(is_macro_expression)
        }
        _ => false,
    }
}
