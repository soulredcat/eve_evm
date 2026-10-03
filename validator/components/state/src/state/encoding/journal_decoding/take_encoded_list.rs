// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{StateError, state::encoding::take_list};

pub(crate) fn take_encoded_list<'a>(
    input: &mut &'a [u8],
    maximum: usize,
) -> Result<&'a [u8], StateError> {
    let before = *input;
    take_list(input)?;
    let length = before.len() - input.len();
    if length > maximum {
        return Err(StateError::BudgetExceeded);
    }
    Ok(&before[..length])
}
