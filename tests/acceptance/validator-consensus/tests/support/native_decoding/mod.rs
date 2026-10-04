// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) use eve_consensus_comet::rpc_decoding::{
    NativeBlock, decode_native_block, decode_native_commit, decode_native_header,
    decode_native_validators,
};
#[cfg(test)]
mod tests;
