// Nested contract with the same name as another contract in the same project (../1/Nested.sol)
// Test must ensure that both are loaded correctly

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract Nested {
    uint256 public numberOfHorses;

    function updateHorseNumber(uint256 newNumberOfHorses) external {
        numberOfHorses = newNumberOfHorses;
    }
}
