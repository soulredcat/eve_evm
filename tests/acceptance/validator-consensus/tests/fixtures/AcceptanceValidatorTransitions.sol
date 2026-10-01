// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity =0.8.37;

/// Disposable genesis fixture. Trailer: authority20 || EVE_B3_ACCEPTANCE_V1 || digest32.
contract AcceptanceValidatorTransitions {
    uint8 public nextAction;
    event Transition(bytes32 indexed fixtureDigest, uint8 indexed action);
    error InvalidFixture();
    error Unauthorized();
    error InvalidAction();

    function transition(uint8 action) external {
        uint256 size = address(this).code.length;
        if (size < 72) revert InvalidFixture();
        bytes memory footer = new bytes(72);
        address authority;
        bytes20 tag;
        bytes32 digest;
        assembly {
            extcodecopy(address(), add(footer, 32), sub(extcodesize(address()), 72), 72)
            authority := shr(96, mload(add(footer, 32)))
            tag := mload(add(footer, 52))
            digest := mload(add(footer, 72))
        }
        if (tag != bytes20("EVE_B3_ACCEPTANCE_V1")) revert InvalidFixture();
        if (msg.sender != authority) revert Unauthorized();
        if (action == 0 || action > 3 || action != nextAction + 1) revert InvalidAction();
        nextAction = action;
        emit Transition(digest, action);
    }
}
