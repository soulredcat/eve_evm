// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Minimal acceptance proxy without upgrade authority or production guarantees.
contract DelegateProxy {
    uint256 public value;

    function execute(address target, bytes memory callData) external returns (bytes memory result) {
        bool success;
        (success, result) = target.delegatecall(callData);
        if (!success) {
            assembly {
                revert(add(result, 0x20), mload(result))
            }
        }
    }
}
