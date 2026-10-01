// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pragma solidity 0.8.37;

import "../tokens/TestERC20.sol";

/// No-fee, two-token acceptance pool with literal integer swap expectations.
/// It deliberately omits liquidity shares and all production DEX guarantees.
contract TestPool {
    TestERC20 public immutable token0;
    TestERC20 public immutable token1;
    uint256 public reserve0;
    uint256 public reserve1;

    event Seed(uint256 amount0, uint256 amount1);
    event Swap(address indexed caller, address indexed tokenIn, uint256 amountIn, uint256 amountOut);

    constructor(TestERC20 first, TestERC20 second) {
        require(address(first) != address(second), "DISTINCT_TOKENS_REQUIRED");
        token0 = first;
        token1 = second;
    }

    function seed(uint256 amount0, uint256 amount1) external {
        require(amount0 > 0 && amount1 > 0, "EMPTY_SEED");
        require(token0.transferFrom(msg.sender, address(this), amount0), "SEED_FIRST_FAILED");
        require(token1.transferFrom(msg.sender, address(this), amount1), "SEED_SECOND_FAILED");
        reserve0 += amount0;
        reserve1 += amount1;
        emit Seed(amount0, amount1);
    }

    function swapExactIn(TestERC20 input, uint256 amountIn, uint256 minimumOut) external returns (uint256 amountOut) {
        require(amountIn > 0 && reserve0 > 0 && reserve1 > 0, "EMPTY_SWAP");
        bool first = address(input) == address(token0);
        require(first || address(input) == address(token1), "UNKNOWN_TOKEN");
        uint256 reserveIn = first ? reserve0 : reserve1;
        uint256 reserveOut = first ? reserve1 : reserve0;
        amountOut = amountIn * reserveOut / (reserveIn + amountIn);
        require(amountOut >= minimumOut && amountOut > 0, "MINIMUM_OUTPUT");
        require(input.transferFrom(msg.sender, address(this), amountIn), "INPUT_TRANSFER_FAILED");
        require((first ? token1 : token0).transfer(msg.sender, amountOut), "OUTPUT_TRANSFER_FAILED");
        if (first) {
            reserve0 = reserveIn + amountIn;
            reserve1 = reserveOut - amountOut;
        } else {
            reserve1 = reserveIn + amountIn;
            reserve0 = reserveOut - amountOut;
        }
        emit Swap(msg.sender, address(input), amountIn, amountOut);
    }
}
