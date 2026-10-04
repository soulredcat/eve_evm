// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    DevelopmentFollowerConfig, validate_development_follower::validate_development_follower,
};
use std::path::PathBuf;

#[test]
fn checkpoint_height_bounds_and_development_acknowledgment_are_checked_before_io() {
    let mut config = DevelopmentFollowerConfig {
        root: PathBuf::from("unused-test-root"),
        data: PathBuf::from("unused-test-data"),
        genesis: PathBuf::from("unused-test-genesis"),
        acknowledge_unsafe_development: true,
        validator_address: "127.0.0.1:26657".parse().unwrap(),
        http_address: "127.0.0.1:0".parse().unwrap(),
        ws_address: "127.0.0.1:0".parse().unwrap(),
        node_name: "checkpoint-guard".into(),
        zone_id: 1,
        poll_interval_ms: 100,
        checkpoint_height: None,
    };
    assert!(validate_development_follower(&config).is_ok());
    for height in [0, 10_001, u64::MAX] {
        config.checkpoint_height = Some(height);
        assert!(
            validate_development_follower(&config)
                .unwrap_err()
                .to_string()
                .contains("checkpoint height")
        );
    }
    for height in [1, 10_000] {
        config.checkpoint_height = Some(height);
        assert!(validate_development_follower(&config).is_ok());
    }
    config.acknowledge_unsafe_development = false;
    assert!(
        validate_development_follower(&config)
            .unwrap_err()
            .to_string()
            .contains("acknowledgment")
    );
}

#[test]
fn checkpoint_cli_argument_is_optional_and_preserves_ordinary_follower_defaults() {
    #[derive(clap::Parser)]
    struct Parser {
        #[command(flatten)]
        config: DevelopmentFollowerConfig,
    }
    use clap::Parser as _;
    let arguments = [
        "eve-checkpoint-argument-test",
        "--root",
        "unused-root",
        "--data",
        "local-tests/follower",
        "--genesis",
        "unused-genesis",
        "--acknowledge-unsafe-development",
        "--validator-address",
        "127.0.0.1:26657",
    ];
    let ordinary = Parser::try_parse_from(arguments).unwrap();
    assert!(ordinary.config.checkpoint_height.is_none());
    let checkpoint =
        Parser::try_parse_from(arguments.into_iter().chain(["--checkpoint-height", "4"])).unwrap();
    assert_eq!(checkpoint.config.checkpoint_height, Some(4));
    assert_eq!(
        checkpoint.config.poll_interval_ms,
        ordinary.config.poll_interval_ms
    );
}
