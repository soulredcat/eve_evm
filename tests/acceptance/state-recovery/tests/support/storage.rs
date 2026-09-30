use eve_state::development_state_budget;
use eve_storage::{recovery::types::StorageBudget, state::StateStorageBudget};
use std::path::{Path, PathBuf};

pub fn budget() -> StateStorageBudget {
    StateStorageBudget {
        database: StorageBudget {
            max_record_bytes: 16 * 1024 * 1024,
            max_batch_bytes: 128 * 1024 * 1024,
            max_batch_records: 4,
            write_buffer_bytes: 4 * 1024 * 1024,
            write_buffer_count: 2,
            block_cache_bytes: 4 * 1024 * 1024,
            max_background_jobs: 2,
            max_open_files: 32,
        },
        logical: development_state_budget(),
        maximum_commit_bytes: 128 * 1024 * 1024,
        maximum_snapshot_bytes: 256 * 1024 * 1024,
        maximum_snapshots: 4,
        maximum_snapshot_files: 1024,
    }
}

pub fn local_directory(label: &str) -> tempfile::TempDir {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap();
    let local = repository.join("local-tests/b1-acceptance");
    std::fs::create_dir_all(&local).unwrap();
    let canonical = local.canonicalize().unwrap();
    assert!(canonical.starts_with(repository.canonicalize().unwrap()));
    assert!(
        !std::fs::symlink_metadata(&local)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    tempfile::Builder::new()
        .prefix(label)
        .tempdir_in(canonical)
        .unwrap()
}

pub fn key(prefix: &[u8], suffix: &[u8]) -> Vec<u8> {
    [prefix, suffix].concat()
}

pub fn height_key(prefix: &[u8], height: u64) -> Vec<u8> {
    key(prefix, &height.to_be_bytes())
}

pub fn database_path(directory: &tempfile::TempDir) -> PathBuf {
    directory.path().join("database")
}
