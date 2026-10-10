// SPDX-License-Identifier: MIT
pragma solidity >=0.6.0 <0.8.0;

contract UnsafeMathPre08Range {
    uint256 public x;

    function add(uint256 y) public {
        x = x + y;
    }
}
