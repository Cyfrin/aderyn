// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract MyDumbEventEmittingContract {
    // Public constant variable
    uint256 public constant MAX_SUPPLY = 1e4;

    event SupplySet(uint256 indexed supply);

    constructor() {
        // It's okay to emit public constants in constructor
        emit SupplySet(MAX_SUPPLY);
    }

    function emitSupplyEvent() external {
        // Not okay to emit public constants in other functions
        emit SupplySet(MAX_SUPPLY);
    }
}
