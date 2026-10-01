// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

import "./TestReentrantTarget.sol";

/// Disposable caller whose final callback can revert every prior call effect.
contract TestReentryDriver is TestCallback {
    TestReentrantTarget public immutable target;
    bool private failAtEnd;

    constructor(TestReentrantTarget calledTarget) {
        target = calledTarget;
    }

    function start(uint256 remaining, bool shouldFail) external {
        failAtEnd = shouldFail;
        target.callExternal(this, remaining);
    }

    function onCall(uint256 remaining) external {
        require(msg.sender == address(target), "ONLY_FIXTURE_TARGET");
        if (remaining > 0) {
            target.callExternal(this, remaining - 1);
        } else {
            require(!failAtEnd, "END_CALLBACK_REVERT");
        }
    }
}
