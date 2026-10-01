// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

/// Creation fixture. The client independently derives the CREATE2 address.
contract Create2Factory {
    event Created(address indexed deployed, bytes32 indexed salt);

    function deploy(bytes memory initCode, bytes32 salt) external returns (address deployed) {
        assembly {
            deployed := create2(0, add(initCode, 0x20), mload(initCode), salt)
        }
        require(deployed != address(0), "CREATE2_FAILED");
        emit Created(deployed, salt);
    }
}
