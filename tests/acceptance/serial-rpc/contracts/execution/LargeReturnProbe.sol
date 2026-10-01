// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Disposable return-data fixture for explicit transport response limits.
contract LargeReturnProbe {
    event Data(bytes value);

    function returnData(uint256 length) external pure returns (bytes memory) {
        require(length <= 2 * 1024 * 1024, "FIXTURE_OUTPUT_BOUND");
        return new bytes(length);
    }

    function emitData(uint256 length) external {
        require(length <= 2 * 1024 * 1024, "FIXTURE_OUTPUT_BOUND");
        emit Data(new bytes(length));
    }
}
