// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Delegatecall storage target; slot zero must belong to the calling proxy.
contract DelegateTarget {
    uint256 public value;

    function setValue(uint256 next) external {
        value = next;
    }
}
