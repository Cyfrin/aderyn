// Nested contract with the same name as another contract in the same project (../2/Nested.sol)
// Test must ensure that both are loaded correctly

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract Nested {
    uint256 numberOfHorses;

    function updateHorseNumber(uint256 newNumberOfHorses) external {
        numberOfHorses = newNumberOfHorses;
    }

    function readNumberOfHorses() external view returns (uint256) {
        return numberOfHorses;
    }
}
