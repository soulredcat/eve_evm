// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    DevelopmentFollowerConfig, open_development_follower::open_development_follower,
    validate_development_follower::validate_development_follower,
};
use std::path::PathBuf;

#[test]
fn unacknowledged_or_nonlocal_follower_is_refused_before_genesis_or_namespace_io() {
    let mut config = DevelopmentFollowerConfig {
        root: PathBuf::from("nonexistent-test-root"),
        data: PathBuf::from("outside-local-tests"),
        genesis: PathBuf::from("missing-test-genesis"),
        acknowledge_unsafe_development: false,
        validator_address: "127.0.0.1:26657".parse().unwrap(),
        http_address: "127.0.0.1:0".parse().unwrap(),
        ws_address: "127.0.0.1:0".parse().unwrap(),
        node_name: "bounded-test".into(),
        zone_id: 1,
        poll_interval_ms: 100,
        checkpoint_height: None,
    };
    let failure = open_development_follower(&config).err().unwrap();
    assert!(failure.to_string().contains("acknowledgment"));
    config.acknowledge_unsafe_development = true;
    config.validator_address = "192.0.2.1:26657".parse().unwrap();
    assert!(
        open_development_follower(&config)
            .err()
            .unwrap()
            .to_string()
            .contains("loopback")
    );
    config.validator_address = "127.0.0.1:26657".parse().unwrap();
    config.node_name = "../foreign".into();
    assert!(validate_development_follower(&config).is_err());
    config.node_name = "bounded-test".into();
    config.poll_interval_ms = 0;
    assert!(validate_development_follower(&config).is_err());
}
