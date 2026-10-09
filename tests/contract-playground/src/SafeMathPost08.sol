// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SafeMathPost08 {
    uint256 public x;

    function add(uint256 y) public {
        x = x + y; // should NOT flag because pragma >= 0.8
        x += y;    // should NOT flag
        x++;       // should NOT flag
    }
}
