use crate::support;

use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};
use support::{genesis::development_spec_json, storage::local_directory};

#[test]
fn tg03_master_sync_only_production_and_unacknowledged_modes_cannot_produce_local_state() {
    let directory = local_directory("master-authority-");
    let spec = directory.path().join("genesis.json");
    std::fs::write(&spec, serde_json::to_vec(&development_spec_json()).unwrap()).unwrap();
    for (index, mode, acknowledged) in [
        (0, "MASTER_SYNC_ONLY", true),
        (1, "PRODUCTION", true),
        (2, "DEV_ALL_IN_ONE", false),
    ] {
        let data = directory.path().join(format!("rejected-{index}"));
        let output = invoke("init-dev", &data, &spec, mode, acknowledged);
        assert!(!output.status.success(), "{mode}");
        assert!(
            !data.exists(),
            "rejected authority must not initialize a namespace"
        );
    }
}

#[test]
fn tg03_production_identity_profile_or_signing_material_is_not_a_development_default() {
    for fault in ["network", "profile", "private-key"] {
        let directory = local_directory("master-profile-");
        let spec = directory.path().join("genesis.json");
        let mut input = development_spec_json();
        match fault {
            "network" => {
                input["network_name"] = "eve-mainnet".into();
                input["evm_chain_id"] = 1.into();
            }
            "profile" => input["profile"] = "PQ_PROFILE_VERIFIED".into(),
            "private-key" => {
                input["validators"][0]["private_key"] =
                    "REJECT_UNRECOGNIZED_TEST_SIGNER_MATERIAL".into()
            }
            _ => unreachable!(),
        }
        std::fs::write(&spec, serde_json::to_vec(&input).unwrap()).unwrap();
        let data = directory.path().join("rejected");
        assert!(
            !invoke("init-dev", &data, &spec, "DEV_ALL_IN_ONE", true)
                .status
                .success(),
            "{fault}"
        );
        assert!(!data.exists());
    }
}

#[test]
fn tg03_real_local_harness_reports_durability_without_validator_finality_authority() {
    let directory = local_directory("master-local-");
    let spec = directory.path().join("genesis.json");
    std::fs::write(&spec, serde_json::to_vec(&development_spec_json()).unwrap()).unwrap();
    let data = directory.path().join("development");
    let output = invoke("init-dev", &data, &spec, "DEV_ALL_IN_ONE", true);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let status: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(status["mode"], "DEV_ALL_IN_ONE");
    assert_eq!(
        status["security_scope"],
        "LOCAL_DURABLE_ONLY_NO_VALIDATOR_FINALITY"
    );
    assert_eq!(status["authenticated_validator_finality"], false);
    assert_eq!(status["height"], 0);
    let inspected = invoke("inspect-dev", &data, &spec, "DEV_ALL_IN_ONE", true);
    assert!(inspected.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&inspected.stdout).unwrap(),
        status
    );
    assert!(
        !invoke("inspect-dev", &data, &spec, "MASTER_SYNC_ONLY", true)
            .status
            .success()
    );
}

fn invoke(operation: &str, data: &Path, spec: &Path, mode: &str, acknowledged: bool) -> Output {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap();
    let mut command = Command::new(binary());
    command
        .arg(operation)
        .arg("--root")
        .arg(root)
        .arg("--data")
        .arg(data)
        .arg("--genesis")
        .arg(spec)
        .arg("--mode")
        .arg(mode);
    if acknowledged {
        command.arg("--acknowledge-unsafe-development");
    }
    command.output().unwrap()
}

fn binary() -> PathBuf {
    let path = std::env::var_os("EVE_MASTER_DEV_BINARY")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join(format!("eve-master{}", std::env::consts::EXE_SUFFIX))
        });
    let metadata = std::fs::symlink_metadata(&path).expect(
        "build the current eve-master development binary before its mandatory acceptance tests",
    );
    assert!(metadata.is_file() && !metadata.file_type().is_symlink());
    path.canonicalize().unwrap()
}
