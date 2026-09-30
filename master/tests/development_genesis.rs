use alloy_primitives::{Address, U256};
use ed25519_dalek::SigningKey;
use eve_master::development::config::decode_development_spec::decode_development_spec;

fn fake_spec() -> serde_json::Value {
    let unit = U256::from(1_000_000_000_000_000_000_u64);
    let mut accounts = Vec::new();
    let mut validators = Vec::new();
    for value in 1_u8..=4 {
        let owner = Address::repeat_byte(value);
        // Deterministic private material exists only in isolated test generation.
        let public = SigningKey::from_bytes(&[value; 32])
            .verifying_key()
            .to_bytes();
        accounts.push(serde_json::json!({"address": owner, "funded_balance": U256::from(20_000) * unit, "nonce": 0, "code": "0x"}));
        validators.push(serde_json::json!({"owner": owner, "classical_public_key": format!("0x{}",hex::encode(public)), "self_bond":U256::from(10_000)*unit,"voting_power":10_000}));
    }
    serde_json::json!({"schema_version":1,"protocol_version":1,"network_name":"eve-local-v1","evm_chain_id":31337,"initial_timestamp":1_700_000_000,"profile":"CLASSICAL_DEV","accounts":accounts,"validators":validators})
}

#[test]
fn development_genesis_accepts_real_public_keys_and_refuses_production_or_key_fields() {
    let value = fake_spec();
    let parsed = decode_development_spec(&serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(parsed.validators.len(), 4);
    for field in ["network_name", "profile", "production_signing_key"] {
        let mut invalid = value.clone();
        invalid[field] = serde_json::json!("PRODUCTION");
        assert!(
            decode_development_spec(&serde_json::to_vec(&invalid).unwrap()).is_err(),
            "{field}"
        );
    }
    let mut bad_key = value;
    bad_key["validators"][0]["classical_public_key"] = serde_json::json!("0x01");
    assert!(decode_development_spec(&serde_json::to_vec(&bad_key).unwrap()).is_err());
}

#[test]
fn development_genesis_refuses_missing_funding_and_oversized_input() {
    let mut invalid = fake_spec();
    invalid["accounts"][0]["funded_balance"] = serde_json::json!("0x0");
    assert!(decode_development_spec(&serde_json::to_vec(&invalid).unwrap()).is_err());
    assert!(decode_development_spec(&vec![b' '; 1_048_577]).is_err());
}

#[test]
fn cli_initializes_reopens_exports_and_restores_real_local_state_without_finality() {
    use std::process::Command;
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().canonicalize().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(&root)
            .status()
            .unwrap()
            .success()
    );
    std::fs::write(root.join(".gitignore"), "/local-tests/\n").unwrap();
    let spec = root.join("development.json");
    std::fs::write(&spec, serde_json::to_vec_pretty(&fake_spec()).unwrap()).unwrap();
    let binary = env!("CARGO_BIN_EXE_eve-master");
    for operation in ["init-dev", "inspect-dev"] {
        let result = Command::new(binary)
            .arg(operation)
            .arg("--root")
            .arg(&root)
            .args(["--data", "local-tests/live", "--genesis"])
            .arg(&spec)
            .args([
                "--mode",
                "DEV_ALL_IN_ONE",
                "--acknowledge-unsafe-development",
            ])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let status: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(status["height"], 0);
        assert_eq!(status["authenticated_validator_finality"], false);
    }
    let block = root.join("block.json");
    std::fs::write(
        &block,
        serde_json::to_vec(&serde_json::json!({"timestamp":1_700_000_001,"transactions":[]}))
            .unwrap(),
    )
    .unwrap();
    let applied = Command::new(binary)
        .arg("apply-dev")
        .arg("--root")
        .arg(&root)
        .args(["--data", "local-tests/live", "--genesis"])
        .arg(&spec)
        .args([
            "--mode",
            "DEV_ALL_IN_ONE",
            "--acknowledge-unsafe-development",
            "--block",
        ])
        .arg(&block)
        .output()
        .unwrap();
    assert!(
        applied.status.success(),
        "{}",
        String::from_utf8_lossy(&applied.stderr)
    );
    let status: serde_json::Value = serde_json::from_slice(&applied.stdout).unwrap();
    assert_eq!(status["height"], 1);
    assert_eq!(status["authenticated_validator_finality"], false);
    let exported = Command::new(binary)
        .arg("snapshot-dev")
        .arg("--root")
        .arg(&root)
        .args(["--data", "local-tests/live", "--genesis"])
        .arg(&spec)
        .args([
            "--mode",
            "DEV_ALL_IN_ONE",
            "--acknowledge-unsafe-development",
            "--output",
            "local-tests/snapshot",
        ])
        .output()
        .unwrap();
    assert!(
        exported.status.success(),
        "{}",
        String::from_utf8_lossy(&exported.stderr)
    );
    let restored = Command::new(binary)
        .arg("restore-dev")
        .arg("--root")
        .arg(&root)
        .args(["--data", "local-tests/restored", "--genesis"])
        .arg(&spec)
        .args([
            "--mode",
            "DEV_ALL_IN_ONE",
            "--acknowledge-unsafe-development",
            "--source",
            "local-tests/snapshot",
        ])
        .output()
        .unwrap();
    assert!(
        restored.status.success(),
        "{}",
        String::from_utf8_lossy(&restored.stderr)
    );
    let status: serde_json::Value = serde_json::from_slice(&restored.stdout).unwrap();
    assert_eq!(status["height"], 1);
    assert_eq!(status["authenticated_validator_finality"], false);
    let repeated = Command::new(binary)
        .arg("init-dev")
        .arg("--root")
        .arg(&root)
        .args(["--data", "local-tests/live", "--genesis"])
        .arg(&spec)
        .args([
            "--mode",
            "DEV_ALL_IN_ONE",
            "--acknowledge-unsafe-development",
        ])
        .output()
        .unwrap();
    assert!(!repeated.status.success());
}
