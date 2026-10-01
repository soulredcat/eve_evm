// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Canonical RLP construction shared by this protocol domain.

mod encode_list;
mod encode_optional_height;

pub(crate) use encode_list::encode_list;
pub(crate) use encode_optional_height::encode_optional_height;
