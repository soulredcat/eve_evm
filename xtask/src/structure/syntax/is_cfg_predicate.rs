// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use syn::{Meta, Token, punctuated::Punctuated};

pub fn is_cfg_predicate(predicate: &Meta) -> bool {
    match predicate {
        Meta::Path(path) => path.get_ident().is_some(),
        Meta::NameValue(value) => {
            let syn::Expr::Lit(literal) = &value.value else {
                return false;
            };
            value.path.get_ident().is_some() && matches!(literal.lit, syn::Lit::Str(_))
        }
        Meta::List(value)
            if value
                .path
                .get_ident()
                .is_some_and(|name| matches!(name.to_string().as_str(), "all" | "any" | "not")) =>
        {
            value
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .is_ok_and(|values| values.iter().all(is_cfg_predicate))
        }
        _ => false,
    }
}
