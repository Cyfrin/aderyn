// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract UnusedStateVariables {
    // Unused state variables (BAD)
    uint256 internal unusedUint256;
    address internal unusedAddress;
    bool private unusedBool;
    string private unusedString;

    // Used state variable (GOOD)
    bytes32 public usedBytes32; // External contracts may want to interact with it by calling it as a function
    uint256 internal usedUint256;

    function setValue(uint256 v) external {
        usedUint256 = v;
    }
}
