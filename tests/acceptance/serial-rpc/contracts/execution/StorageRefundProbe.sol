// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Disposable fixture for storage refunds, revert data and out-of-gas isolation.
contract StorageRefundProbe {
    uint256[16] public values;

    error DeliberateFailure(uint256 value);

    function populate() external {
        for (uint256 index = 0; index < values.length; index++) {
            values[index] = index + 1;
        }
    }

    function clear() external {
        for (uint256 index = 0; index < values.length; index++) {
            delete values[index];
        }
    }

    function failAfterWrite(uint256 next) external {
        values[0] = next;
        revert DeliberateFailure(next);
    }

    function exhaustGas() external {
        uint256 index;
        while (true) {
            values[index % values.length] = index;
            index++;
        }
    }
}
