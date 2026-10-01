// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

import "../pools/TestPool.sol";

/// Disposable two-leg router used to observe transaction-wide EVM rollback.
contract AtomicSwapRouter {
    TestERC20 public immutable firstToken;
    TestERC20 public immutable middleToken;
    TestERC20 public immutable lastToken;
    TestPool public immutable firstPool;
    TestPool public immutable secondPool;

    event Routed(address indexed caller, address indexed recipient, uint256 amountIn, uint256 amountOut);

    constructor(TestERC20 first, TestERC20 middle, TestERC20 last, TestPool firstMarket, TestPool secondMarket) {
        firstToken = first;
        middleToken = middle;
        lastToken = last;
        firstPool = firstMarket;
        secondPool = secondMarket;
    }

    function swapTwo(uint256 amountIn, uint256 minimumFirst, uint256 minimumSecond, address recipient)
        external
        returns (uint256 amountOut)
    {
        require(firstToken.transferFrom(msg.sender, address(this), amountIn), "ROUTER_INPUT_FAILED");
        require(firstToken.approve(address(firstPool), amountIn), "FIRST_APPROVAL_FAILED");
        uint256 middleAmount = firstPool.swapExactIn(firstToken, amountIn, minimumFirst);
        require(middleToken.approve(address(secondPool), middleAmount), "SECOND_APPROVAL_FAILED");
        amountOut = secondPool.swapExactIn(middleToken, middleAmount, minimumSecond);
        require(lastToken.transfer(recipient, amountOut), "ROUTER_OUTPUT_FAILED");
        emit Routed(msg.sender, recipient, amountIn, amountOut);
    }
}
