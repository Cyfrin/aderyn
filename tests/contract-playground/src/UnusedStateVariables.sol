// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract UnusedStateVariables {
    // Unused state variables (BAD)
    uint256 public unusedUint256;
    address public unusedAddress;
    bool public unusedBool;
    string public unusedString;
    bytes32 public unusedBytes32;

    // Used state variable (GOOD)
    uint256 public usedUint256;

    function setValue(uint256 v) external {
        usedUint256 = v;
    }
}
