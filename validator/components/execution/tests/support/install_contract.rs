// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, Bytes};
use eve_evm::InMemoryDB;
use revm::{bytecode::Bytecode, state::AccountInfo};

pub fn install_contract(state: &mut InMemoryDB, address: Address, runtime: &[u8]) {
    let bytecode = Bytecode::new_raw(Bytes::copy_from_slice(runtime));
    state.insert_account_info(
        address,
        AccountInfo {
            nonce: 1,
            code_hash: bytecode.hash_slow(),
            code: Some(bytecode),
            ..Default::default()
        },
    );
}
