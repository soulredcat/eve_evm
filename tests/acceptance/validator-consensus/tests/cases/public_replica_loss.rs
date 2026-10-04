// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod relocate_owned_replica;
mod start_replica_pair;
mod verify_preserved_replica;
mod verify_replica_pair;
mod verify_replica_state;
mod verify_replica_status;

use super::{
    public_checkpoint::create_checkpoint_follower, public_follower::stop_public_follower,
    public_tail_recovery::kill_public_follower,
};
use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Result, ensure};
use std::collections::BTreeMap;

/// SIMULATED_CONFIGURED_REPLICA_DATA_LOSS: both actual public nodes lose access
/// to configured data. Original bytes remain preserved under unused private names.
#[test]
fn t_g06_all_configured_public_replicas_recover_from_durable_validators_with_masters_absent_slice()
-> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("public-replica-loss", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x95);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    ensure!(
        first_height > 0 && first_height <= 128,
        "PUBLIC_REPLICA_FIRST_HEIGHT_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let first_history = collect_certified_history(&cluster, 0, first_height + 1, &BTreeMap::new())?;
    let first_oracle = replay_history(&cluster, &first_history)?;
    ensure!(
        first_oracle[first_height as usize]
            .block
            .transactions
            .contains(&first),
        "PUBLIC_REPLICA_FIRST_TRANSACTION_MISSING"
    );
    let (first_namespace, first_replica) = create_checkpoint_follower(&cluster)?;
    let (second_namespace, second_replica) = create_checkpoint_follower(&cluster)?;
    let mut replicas = [first_replica, second_replica];
    ensure!(
        replicas[0].repository_root != replicas[1].repository_root
            && replicas[0].data != replicas[1].data
            && replicas[0].binary == replicas[1].binary
            && replicas[0].genesis == replicas[1].genesis
            && replicas[0].validator == replicas[1].validator,
        "PUBLIC_REPLICA_ISOLATED_ROOTS_AND_IDENTICAL_CHAIN_SOURCE"
    );
    for left in [replicas[0].http, replicas[0].ws] {
        ensure!(
            left != replicas[1].http && left != replicas[1].ws,
            "PUBLIC_REPLICA_DISTINCT_LISTENER_PORTS"
        );
    }
    start_replica_pair::start_replica_pair(&mut replicas)?;
    verify_replica_pair::verify_replica_pair(
        &cluster,
        &mut replicas,
        recipient,
        first_height as u64,
        1,
    )?;
    for replica in &mut replicas {
        kill_public_follower(replica)?;
    }
    ensure!(
        replicas.iter().all(|replica| replica.child.is_none()),
        "PUBLIC_REPLICA_CHILDREN_NOT_REAPED"
    );
    let lost_first =
        relocate_owned_replica::relocate_owned_replica(&replicas[0], first_namespace.path())?;
    let lost_second =
        relocate_owned_replica::relocate_owned_replica(&replicas[1], second_namespace.path())?;
    ensure!(
        replicas.iter().all(|replica| !replica.data.exists()),
        "PUBLIC_REPLICA_CONFIGURED_DATA_NOT_LOST"
    );
    // All public children and every master are absent during this new finality.
    // The existing real validator repositories retain the recovery source.
    let second = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let second_height = submit_transaction(&cluster, 0, &second)?;
    ensure!(
        second_height > first_height && second_height <= 128,
        "PUBLIC_REPLICA_SECOND_HEIGHT_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], second_height + 2)?;
    let second_history =
        collect_certified_history(&cluster, 0, second_height + 1, &BTreeMap::new())?;
    let second_oracle = replay_history(&cluster, &second_history)?;
    ensure!(
        second_oracle[first_height as usize] == first_oracle[first_height as usize],
        "PUBLIC_REPLICA_CERTIFIED_PREFIX_CHANGED"
    );
    ensure!(
        second_oracle[second_height as usize]
            .block
            .transactions
            .contains(&second),
        "PUBLIC_REPLICA_SECOND_TRANSACTION_MISSING"
    );
    start_replica_pair::start_replica_pair(&mut replicas)?;
    verify_replica_pair::verify_replica_pair(
        &cluster,
        &mut replicas,
        recipient,
        second_height as u64,
        2,
    )?;
    verify_preserved_replica::verify_preserved_replica(&replicas[0], &lost_first)?;
    verify_preserved_replica::verify_preserved_replica(&replicas[1], &lost_second)?;
    for replica in &mut replicas {
        stop_public_follower(replica)?;
    }
    cluster.stop()?;
    // Only the test TempDir owners remove their disposable namespaces at exit.
    drop(replicas);
    drop((first_namespace, second_namespace));
    Ok(())
}
