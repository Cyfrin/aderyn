// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract SimpleProgram {
    function function1(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
    }
}
