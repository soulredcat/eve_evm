// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Shanghai-only fixture: a later-transaction SELFDESTRUCT deletes this state.
contract ShanghaiDestruction {
    uint256 public value = 77;

    constructor() payable {}

    function destroy(address payable beneficiary) external {
        selfdestruct(beneficiary);
    }
}
