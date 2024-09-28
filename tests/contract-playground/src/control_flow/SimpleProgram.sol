// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract SimpleProgram {
    function function1(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
    }

    function function2(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
        {
            c = start * times;
            c %= mod;
        }
        mod = start - times; // TODO: Debug why this statement is missing in the control flow graph
        c = start * times;
        c = start * times;
    }
}
