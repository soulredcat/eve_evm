// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    set_engine_configuration_value, validate_engine_public_genesis, validate_engine_socket_paths,
    validate_persistent_engine_peers,
};
use crate::development::{
    config::DevelopmentValidatorConfig,
    engine::{
        home::{validate_engine_home, write_owned_engine_file},
        types::{MARKER_NAME, VerifiedEngineBinary},
        verification::{hash_engine_file, read_engine_file},
    },
};
use anyhow::Result;
use std::path::Path;

pub(in crate::development::engine) fn configure_engine_home(
    config: &DevelopmentValidatorConfig,
    home: &Path,
    application_socket: &Path,
    signer_socket: &Path,
    binary: &VerifiedEngineBinary,
) -> Result<()> {
    validate_engine_socket_paths(&config.data, application_socket, signer_socket)?;
    validate_persistent_engine_peers(&config.persistent_peers)?;
    let mut marker = validate_engine_home(home, binary)?;
    validate_engine_public_genesis(&read_engine_file(
        &home.join("config/genesis.json"),
        1_048_576,
    )?)?;
    let bytes = read_engine_file(&home.join("config/config.toml"), 65_536)?;
    let mut native: toml::Value = toml::from_str(std::str::from_utf8(&bytes)?)?;
    for (path, value) in [
        (
            vec!["proxy_app"],
            toml::Value::String(format!("unix://{}", application_socket.display())),
        ),
        (
            vec!["priv_validator_laddr"],
            toml::Value::String(format!("unix://{}", signer_socket.display())),
        ),
        (vec!["abci"], "socket".into()),
        (vec!["genesis_file"], "config/genesis.json".into()),
        (
            vec!["priv_validator_key_file"],
            "config/priv_validator_key.json".into(),
        ),
        (
            vec!["priv_validator_state_file"],
            "data/priv_validator_state.json".into(),
        ),
        (vec!["node_key_file"], "config/node_key.json".into()),
        (vec!["db_backend"], "goleveldb".into()),
        (vec!["db_dir"], "data".into()),
        (vec!["log_level"], "error".into()),
        (
            vec!["rpc", "laddr"],
            format!("tcp://{}", config.rpc_address).into(),
        ),
        (vec!["rpc", "unsafe"], false.into()),
        (vec!["rpc", "grpc_laddr"], "".into()),
        (vec!["rpc", "pprof_laddr"], "".into()),
        (vec!["rpc", "max_open_connections"], 64.into()),
        (vec!["rpc", "max_subscription_clients"], 16.into()),
        (vec!["rpc", "max_subscriptions_per_client"], 4.into()),
        (
            vec!["rpc", "experimental_subscription_buffer_size"],
            100.into(),
        ),
        (
            vec!["rpc", "experimental_websocket_write_buffer_size"],
            100.into(),
        ),
        (
            vec!["rpc", "max_body_bytes"],
            (4_i64 * 1_048_576 + 65_536).into(),
        ),
        (vec!["rpc", "max_header_bytes"], 16_384.into()),
        (vec!["rpc", "max_request_batch_size"], 10.into()),
        (
            vec!["p2p", "laddr"],
            format!("tcp://{}", config.p2p_address).into(),
        ),
        (
            vec!["p2p", "persistent_peers"],
            config.persistent_peers.clone().into(),
        ),
        (vec!["p2p", "seeds"], "".into()),
        (
            vec!["p2p", "external_address"],
            config
                .advertised_p2p_address
                .map(|address| address.to_string())
                .unwrap_or_default()
                .into(),
        ),
        (vec!["p2p", "seed_mode"], false.into()),
        (vec!["p2p", "pex"], false.into()),
        (vec!["p2p", "addr_book_strict"], false.into()),
        (vec!["p2p", "allow_duplicate_ip"], true.into()),
        (vec!["p2p", "max_num_inbound_peers"], 16.into()),
        (vec!["p2p", "max_num_outbound_peers"], 16.into()),
        (vec!["mempool", "type"], "flood".into()),
        (vec!["mempool", "size"], 4096.into()),
        (
            vec!["mempool", "max_txs_bytes"],
            (16_i64 * 1_048_576).into(),
        ),
        (vec!["mempool", "max_tx_bytes"], 131_072.into()),
        (vec!["mempool", "cache_size"], 8192.into()),
        (vec!["consensus", "create_empty_blocks"], true.into()),
    ] {
        set_engine_configuration_value(&mut native, &path, value)?;
    }
    let bytes = toml::to_string(&native)?.into_bytes();
    write_owned_engine_file(&home.join("config/config.toml"), &bytes, 65_536)?;
    marker.configuration_sha256 =
        hex::encode(hash_engine_file(&home.join("config/config.toml"), 65_536)?);
    write_owned_engine_file(&home.join(MARKER_NAME), &serde_json::to_vec(&marker)?, 4096)?;
    Ok(())
}
