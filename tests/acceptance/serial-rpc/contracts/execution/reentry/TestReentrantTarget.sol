// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

interface TestCallback {
    function onCall(uint256 remaining) external;
}

/// Deliberately permits recursion to test call ordering and atomic rollback.
contract TestReentrantTarget {
    uint256 public calls;

    function callExternal(TestCallback recipient, uint256 remaining) external {
        calls++;
        recipient.onCall(remaining);
    }
}
