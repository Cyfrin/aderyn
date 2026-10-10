// SPDX-License-Identifier: MIT
pragma solidity 0.4.26;

contract UnsafeMathPre08Exact0426 {
    uint256 public x;

    function add(uint256 y) public {
        x = x + y;
    }
}
