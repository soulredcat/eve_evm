// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::recovery::types::{
    StorageBudget, StorageIdentity, StorageNetworkId, StoredBlockInput,
};

pub fn identity() -> StorageIdentity {
    StorageIdentity {
        network: StorageNetworkId([1; 32]),
        genesis_hash: [2; 32],
        base_height: 0,
        base_block_hash: [3; 32],
    }
}

pub fn budget() -> StorageBudget {
    StorageBudget {
        max_record_bytes: 4096,
        max_batch_bytes: 65536,
        max_batch_records: 16,
        write_buffer_bytes: 4 * 1024 * 1024,
        write_buffer_count: 2,
        block_cache_bytes: 4 * 1024 * 1024,
        max_background_jobs: 2,
        max_open_files: 32,
    }
}

pub fn record(height: u64) -> StoredBlockInput {
    let parent_block_hash = if height == 1 {
        identity().base_block_hash
    } else {
        [u8::try_from(height - 1).expect("small fixture"); 32]
    };
    StoredBlockInput {
        identity: identity(),
        parent_height: height - 1,
        height,
        parent_block_hash,
        block_hash: [u8::try_from(height).expect("small fixture"); 32],
        payload: vec![0xa5, 0x5a],
    }
}
