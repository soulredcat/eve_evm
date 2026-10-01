// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    is_bounded_pattern::is_bounded_pattern, parse_matches_arguments_adapter::MatchesArguments,
};
use crate::structure::syntax::is_macro_expression::is_macro_expression;
pub fn is_bounded_matches(operation: &syn::Macro) -> bool {
    syn::parse2::<MatchesArguments>(operation.tokens.clone()).is_ok_and(|arguments| {
        is_macro_expression(&arguments.value)
            && is_bounded_pattern(&arguments.pattern)
            && arguments.guard.as_ref().is_none_or(is_macro_expression)
    })
}
