// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{Address, Bytes, DevelopmentGenesis, GenesisAccount, U256};

pub fn funded_genesis() -> DevelopmentGenesis {
    let mut genesis = super::genesis();
    genesis.accounts.push(GenesisAccount {
        address: crate::transactions::sender(),
        funded_balance: U256::from(10_u64).pow(U256::from(22)),
        nonce: 0,
        code: Bytes::new(),
    });
    genesis.accounts.push(GenesisAccount {
        address: Address::with_last_byte(0x42),
        funded_balance: U256::ZERO,
        nonce: 0,
        code: "0x606360005500".parse().unwrap(),
    });
    genesis
}
