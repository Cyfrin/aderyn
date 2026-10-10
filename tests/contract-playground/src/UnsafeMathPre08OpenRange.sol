// SPDX-License-Identifier: MIT
pragma solidity >=0.6.0;

contract UnsafeMathPre08OpenRange {
    uint256 public x;

    function add(uint256 y) public {
        x = x + y;
    }
}
