// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::Bytes;

/// Exact existing trie-v1 signed execution vector; the sender is an unsafe test identity.
pub fn signed_transaction() -> Bytes {
    "0xf866808477359400830186a0940000000000000000000000000000000000000042808082f4f5a02c1d1a7db8b28ed638c4c0a70b8789ae4fc8d41fbf881cca9ccb0a97f8f487aba00a5013ea38ccecec4dab4c2c82674109c7a303ad69772b56ef82a47eef05d4c6"
        .parse()
        .unwrap()
}
