// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{fixture, rebind};
use crate::development::acceptance::enabled::load_acceptance_fixture;

#[test]
fn rotation_preserves_owner_power_and_ordered_leave_jail_shapes() {
    let material = fixture();
    let capability = material.load();
    assert_eq!(
        capability.transitions.keys().copied().collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(
        capability.transitions[&1]
            .iter()
            .map(|update| update.power)
            .collect::<Vec<_>>(),
        vec![0, 10000]
    );
    assert_eq!(capability.transitions[&2][0].power, 0);
    assert_eq!(capability.transitions[&3][0].power, 0);
    for mutation in 0..6 {
        let mut material = fixture();
        match mutation {
            0 => material.document["transitions"][0]["updates"][1]["power"] = 10001.into(),
            1 => {
                material.document["transitions"][0]["updates"][0]["public_key"] =
                    hex::encode(material.genesis.validators[1].classical_public_key).into()
            }
            2 => material.document["transitions"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            3 => material.document["transitions"][1]["kind"] = "rotate".into(),
            4 => {
                material.document["transitions"][2]["updates"][0]["public_key"] =
                    hex::encode(material.genesis.validators[1].classical_public_key).into()
            }
            _ => {
                material.document["transitions"][0]["updates"][1]["public_key"] =
                    hex::encode(material.genesis.validators[0].classical_public_key).into()
            }
        }
        rebind(&mut material);
        assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    }
}
