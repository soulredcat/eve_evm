// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Read-only environment fixture; randomness is not asserted to be unbiased.
contract EnvironmentProbe {
    function environment(uint256 historicalHeight)
        external
        view
        returns (uint256, uint256, uint256, uint256, address, uint256, bytes32, uint256)
    {
        return (
            block.number,
            block.chainid,
            block.timestamp,
            block.basefee,
            block.coinbase,
            block.prevrandao,
            blockhash(historicalHeight),
            block.gaslimit
        );
    }
}
