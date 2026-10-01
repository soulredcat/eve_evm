// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{ClusterOptions, cluster::Node};
use alloy_primitives::Address;
use anyhow::{Context, Result};
use ed25519_dalek::SigningKey;
use eve_consensus_comet::consensus::certificates::validator_address;
use k256::elliptic_curve::rand_core::{OsRng, RngCore};

pub(super) fn build_fixture_configuration(
    nodes: &[Node],
    options: &ClusterOptions,
    authority: Address,
) -> Result<serde_json::Value> {
    let future = if nodes.len() == 5 {
        nodes[4].public_key
    } else {
        let mut seed = [0; 32];
        OsRng.fill_bytes(&mut seed);
        SigningKey::from_bytes(&seed).verifying_key().to_bytes()
    };
    let proposer = nodes
        .iter()
        .take(4)
        .min_by_key(|node| validator_address(&node.public_key))
        .context("initial native proposer")?;
    let poison = if options.poison {
        serde_json::json!({
            "height": 1,
            "public_key": hex::encode(proposer.public_key)
        })
    } else {
        serde_json::Value::Null
    };
    Ok(serde_json::json!({
        "version": 1,
        "contract_address": format!("0x{}", super::TRANSITION_ADDRESS),
        "authority": authority,
        "future_validators": [{
            "public_key": hex::encode(future),
            "owner": nodes[0].owner
        }],
        "transitions": [
            {
                "action": 1,
                "kind": "rotate",
                "updates": [
                    {
                        "public_key": hex::encode(nodes[0].public_key),
                        "power": 0
                    },
                    {
                        "public_key": hex::encode(future),
                        "power": 10000
                    }
                ]
            },
            {
                "action": 2,
                "kind": "leave",
                "updates": [{
                    "public_key": hex::encode(nodes[1].public_key),
                    "power": 0
                }]
            },
            {
                "action": 3,
                "kind": "jail",
                "updates": [{
                    "public_key": hex::encode(nodes[2].public_key),
                    "power": 0
                }]
            }
        ],
        "poison": poison
    }))
}
